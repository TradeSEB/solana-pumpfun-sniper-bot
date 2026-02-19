use anyhow::{Context, Result};
use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;
use tokio_stream::StreamExt;

use crate::config::{Config, PUMPFUN_PROGRAM_ID};
use crate::instructions::{extract_create_accounts, CreateAccounts};

/// Token creation event detected from Pump.fun
#[derive(Debug, Clone)]
pub struct TokenCreationEvent {
    pub mint: Pubkey,
    pub bonding_curve: Pubkey,
    pub creator: Pubkey,
    pub signature: String,
    pub slot: u64,
    pub timestamp: i64,
}

/// Token detector using Yellowstone Geyser gRPC or WebSocket fallback
pub struct TokenDetector {
    config: Config,
    pumpfun_program_id: Pubkey,
}

impl TokenDetector {
    pub fn new(config: Config) -> Result<Self> {
        let pumpfun_program_id = Pubkey::from_str(PUMPFUN_PROGRAM_ID)
            .context("Failed to parse Pump.fun program ID")?;

        Ok(Self {
            config,
            pumpfun_program_id,
        })
    }

    /// Start detecting new token creations
    /// 
    /// Returns a stream of TokenCreationEvent
    pub async fn start_detection(
        &self,
    ) -> Result<tokio_stream::wrappers::ReceiverStream<TokenCreationEvent>> {
        // Try Yellowstone Geyser gRPC first if configured
        if let Some(ref grpc_url) = self.config.yellowstone_grpc_url {
            log::info!("Attempting to connect to Yellowstone Geyser gRPC: {}", grpc_url);
            match self.start_geyser_stream(grpc_url).await {
                Ok(stream) => {
                    log::info!("Successfully connected to Yellowstone Geyser");
                    return Ok(stream);
                }
                Err(e) => {
                    log::warn!("Failed to connect to Yellowstone Geyser: {}", e);
                    if !self.config.use_websocket_fallback {
                        return Err(e);
                    }
                    log::info!("Falling back to WebSocket subscription");
                }
            }
        }

        // Fallback to WebSocket subscription
        log::info!("Using WebSocket subscription as detection method");
        self.start_websocket_subscription().await
    }

    /// Start Yellowstone Geyser gRPC stream
    async fn start_geyser_stream(
        &self,
        grpc_url: &str,
    ) -> Result<tokio_stream::wrappers::ReceiverStream<TokenCreationEvent>> {
        use yellowstone_grpc::{
            geyser::SubscribeRequest,
            proto::geyser::SubscribeRequestFilterAccounts,
        };

        let (tx, rx) = tokio::sync::mpsc::channel(1000);

        // Create gRPC client
        let mut client = yellowstone_grpc::GeyserGrpcClient::connect(grpc_url)
            .await
            .context("Failed to connect to Yellowstone Geyser")?;

        // Subscribe to Pump.fun program transactions
        let filter = SubscribeRequestFilterAccounts {
            account: vec![self.pumpfun_program_id.to_string()],
            owner: vec![],
            filters: vec![],
        };

        let request = SubscribeRequest {
            slots: vec![],
            accounts: vec![filter],
            transactions: vec![],
            transactions_status: vec![],
            blocks: vec![],
            blocks_meta: vec![],
            accounts_data_slice: vec![],
            commitment: Some(yellowstone_grpc::proto::geyser::CommitmentLevel::Confirmed as i32),
        };

        // Spawn task to handle stream
        let mut stream = client
            .subscribe_once(request)
            .await
            .context("Failed to subscribe to Geyser stream")?;

        let pumpfun_program_id = self.pumpfun_program_id;
        let config = self.config.clone();

        tokio::spawn(async move {
            while let Some(msg) = stream.message().await.transpose() {
                match msg {
                    Ok(update) => {
                        // Parse transaction update
                        if let Some(tx_update) = update.transaction {
                            if let Some(event) = Self::parse_transaction_update(&tx_update, &pumpfun_program_id) {
                                if let Err(e) = tx.send(event).await {
                                    log::error!("Failed to send token creation event: {}", e);
                                    break;
                                }
                            }
                        }
                    }
                    Err(e) => {
                        log::warn!("Error receiving Geyser update: {}", e);
                    }
                }
            }
        });

        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    /// Start WebSocket subscription (fallback method)
    async fn start_websocket_subscription(
        &self,
    ) -> Result<tokio_stream::wrappers::ReceiverStream<TokenCreationEvent>> {
        use solana_client::nonblocking::rpc_client::RpcClient;

        let (tx, rx) = tokio::sync::mpsc::channel(1000);
        let rpc_url = self.config.rpc_url.clone();
        let pumpfun_program_id = self.pumpfun_program_id;

        // Spawn task to poll for new transactions
        let config_clone = self.config.clone();
        tokio::spawn(async move {
            let client = RpcClient::new(rpc_url);
            let mut last_signature: Option<String> = None;

            loop {
                // Get recent signatures for the program
                match client
                    .get_signatures_for_address(&pumpfun_program_id)
                    .await
                {
                    Ok(signatures) => {
                        for sig_info in signatures.iter().take(10) {
                            // Skip if we've already processed this
                            if let Some(ref last) = last_signature {
                                if sig_info.signature == *last {
                                    break;
                                }
                            }

                            // Parse transaction
                            if let Ok(tx_data) = client.get_transaction(&sig_info.signature, solana_transaction_status::UiTransactionEncoding::Json).await {
                                if let Some(event) = Self::parse_transaction(&tx_data, &pumpfun_program_id, &sig_info.signature) {
                                    if let Err(e) = tx.send(event).await {
                                        log::error!("Failed to send token creation event: {}", e);
                                    }
                                }
                            }
                        }

                        if let Some(first) = signatures.first() {
                            last_signature = Some(first.signature.clone());
                        }
                    }
                    Err(e) => {
                        log::warn!("Error fetching signatures: {}", e);
                    }
                }

                // Rate limiting
                tokio::time::sleep(tokio::time::Duration::from_millis(
                    config_clone.rate_limit_ms,
                ))
                .await;
            }
        });

        Ok(tokio_stream::wrappers::ReceiverStream::new(rx))
    }

    /// Parse transaction update from Geyser
    fn parse_transaction_update(
        _update: &yellowstone_grpc::proto::geyser::TransactionUpdate,
        _program_id: &Pubkey,
    ) -> Option<TokenCreationEvent> {
        // Parse Geyser transaction update
        // This is a simplified version - actual implementation depends on Geyser message format
        // You'll need to:
        // 1. Extract transaction data
        // 2. Check for Create instruction discriminator
        // 3. Extract accounts (mint, bonding_curve, creator)
        // 4. Return TokenCreationEvent
        
        // Placeholder - implement based on actual Geyser message structure
        None
    }

    /// Parse transaction from RPC
    fn parse_transaction(
        tx: &solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta,
        program_id: &Pubkey,
        signature: &str,
    ) -> Option<TokenCreationEvent> {
        use solana_transaction_status::UiTransactionEncoding;

        // Extract transaction data
        let transaction = match tx.transaction {
            solana_transaction_status::EncodedTransaction::Json(ref json_tx) => {
                json_tx
            }
            _ => return None,
        };

        // Find instructions for Pump.fun program
        if let Some(ref message) = transaction.message {
            if let Some(ref account_keys) = message.account_keys {
                // Find program index
                let program_index = account_keys
                    .iter()
                    .position(|key| key == program_id.to_string())?;

                // Find Create instruction
                if let Some(ref instructions) = message.instructions {
                    for ix in instructions {
                        if let Some(program_id_index) = ix.program_id_index {
                            if program_id_index as usize == program_index {
                                // Check if this is a Create instruction
                                if let Some(create_accounts) = Self::parse_create_instruction(
                                    &ix.data,
                                    account_keys,
                                    &ix.accounts,
                                ) {
                                    return Some(TokenCreationEvent {
                                        mint: create_accounts.mint,
                                        bonding_curve: create_accounts.bonding_curve,
                                        creator: create_accounts.creator,
                                        signature: signature.to_string(),
                                        slot: tx.slot.unwrap_or(0),
                                        timestamp: chrono::Utc::now().timestamp(),
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }

        None
    }

    /// Parse Create instruction from instruction data
    fn parse_create_instruction(
        data: &str,
        account_keys: &[String],
        account_indices: &[u8],
    ) -> Option<CreateAccounts> {
        // Decode base58 instruction data
        let decoded = bs58::decode(data).into_vec().ok()?;

        // Check discriminator
        if decoded.len() < 8 {
            return None;
        }

        let discriminator = &decoded[0..8];
        let create_discriminator = [24, 30, 200, 40, 5, 28, 7, 119];
        if discriminator != create_discriminator {
            return None;
        }

        // Extract accounts (order depends on instruction format)
        // This is simplified - verify with actual IDL
        if account_indices.len() < 4 {
            return None;
        }

        let mint = Pubkey::from_str(account_keys.get(account_indices[0] as usize)?).ok()?;
        let bonding_curve = Pubkey::from_str(account_keys.get(account_indices[1] as usize)?).ok()?;
        let creator = Pubkey::from_str(account_keys.get(account_indices[2] as usize)?).ok()?;

        Some(CreateAccounts {
            mint,
            bonding_curve,
            creator,
        })
    }
}
