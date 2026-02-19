use anyhow::{Context, Result};
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    compute_budget::ComputeBudgetInstruction,
    pubkey::Pubkey,
    signature::Signer,
    system_program,
    transaction::VersionedTransaction,
};
use std::str::FromStr;
use tokio::time::{sleep, Duration};

use crate::config::Config;
use crate::detector::TokenCreationEvent;
use crate::instructions::build_buy_instruction;
use crate::utils;
use crate::wallet::Wallet;
// For now, using a placeholder

/// Sniper that evaluates and executes buys on new tokens
pub struct Sniper {
    rpc_client: RpcClient,
    wallet: Wallet,
    config: Config,
}

impl Sniper {
    pub fn new(rpc_url: String, wallet: Wallet, config: Config) -> Self {
        let rpc_client = RpcClient::new_with_commitment(
            rpc_url,
            CommitmentConfig::confirmed(),
        );

        Self {
            rpc_client,
            wallet,
            config,
        }
    }

    /// Evaluate if a token should be sniped based on filters
    pub async fn evaluate_token(&self, event: &TokenCreationEvent) -> Result<bool> {
        log::info!(
            "Evaluating new token: mint={}, creator={}",
            event.mint,
            event.creator
        );

        // Check blacklist
        let creator_str = event.creator.to_string();
        if self.config.blacklisted_creators.contains(&creator_str) {
            log::info!("Token creator is blacklisted: {}", creator_str);
            return Ok(false);
        }

        // Check initial liquidity (if available)
        // Note: This would require fetching bonding curve account data
        // For now, we'll skip this check or make it optional
        if let Err(e) = self.check_liquidity(&event.bonding_curve).await {
            log::warn!("Failed to check liquidity: {}", e);
            // Continue anyway - liquidity check is optional
        }

        // Check metadata (name/symbol) if available
        // This would require fetching token metadata
        // For now, we'll proceed

        log::info!("Token passed all filters: {}", event.mint);
        Ok(true)
    }

    /// Check if token meets liquidity requirements
    async fn check_liquidity(&self, _bonding_curve: &Pubkey) -> Result<()> {
        // Fetch bonding curve account data
        // Parse to get initial liquidity
        // Compare against min/max thresholds
        
        // Placeholder - implement based on bonding curve account structure
        // You'll need to:
        // 1. Fetch account data
        // 2. Deserialize bonding curve account
        // 3. Extract virtual/real SOL reserves
        // 4. Check against config thresholds
        
        Ok(())
    }

    /// Execute a buy on a token
    pub async fn execute_buy(&self, event: &TokenCreationEvent) -> Result<String> {
        if self.config.dry_run {
            log::info!(
                "[DRY RUN] Would buy token: mint={}, amount={} SOL",
                event.mint,
                self.config.buy_amount_sol
            );
            return Ok("dry_run_simulation".to_string());
        }

        log::info!(
            "Executing buy: mint={}, amount={} SOL",
            event.mint,
            self.config.buy_amount_sol
        );

        // Get latest blockhash
        let (blockhash, _) = self
            .rpc_client
            .get_latest_blockhash()
            .await
            .context("Failed to get latest blockhash")?;

        // Build buy instruction
        // Note: This is simplified - you'll need to:
        // 1. Derive associated token account
        // 2. Get SOL reserves account
        // 3. Calculate min tokens out with slippage
        // 4. Build the actual instruction
        
        let buy_amount_lamports = utils::sol_to_lamports(self.config.buy_amount_sol);
        
        // Derive associated bonding curve
        let associated_bonding_curve = self.derive_associated_bonding_curve(&event.bonding_curve)?;
        
        // Get associated token account for buyer
        let buyer_token_account = self.get_or_create_token_account(&event.mint).await?;
        
        // Get SOL reserves (this is typically a PDA)
        let sol_reserves = self.derive_sol_reserves(&event.bonding_curve)?;
        
        // Calculate min tokens out (simplified - should use bonding curve formula)
        let min_tokens_out = 0; // Calculate based on bonding curve
        
        let buy_ix = build_buy_instruction(
            &self.wallet.pubkey(),
            &event.bonding_curve,
            &associated_bonding_curve,
            &event.mint,
            &sol_reserves,
            &event.mint, // Token mint
            buy_amount_lamports,
            min_tokens_out,
        )?;

        // Build transaction
        let mut transaction = solana_sdk::transaction::Transaction::new_with_payer(
            &[buy_ix],
            Some(&self.wallet.pubkey()),
        );
        transaction.sign(&[self.wallet.keypair()], blockhash);

        // Add priority fee
        let priority_fee = utils::estimate_priority_fee(
            &self.rpc_client,
            self.config.priority_fee_micro_lamports,
        )
        .await;

        // Convert to VersionedTransaction if needed
        // For now, use regular transaction
        let versioned_tx = VersionedTransaction::from(transaction);

        // Send with retry
        self.send_transaction_with_retry(versioned_tx, 3).await
    }

    /// Derive associated bonding curve address
    fn derive_associated_bonding_curve(&self, bonding_curve: &Pubkey) -> Result<Pubkey> {
        // This is a placeholder - actual derivation depends on Pump.fun program
        // You may need to use findProgramAddress or similar
        Ok(*bonding_curve) // Simplified
    }

    /// Derive SOL reserves PDA
    fn derive_sol_reserves(&self, bonding_curve: &Pubkey) -> Result<Pubkey> {
        // Placeholder - derive actual PDA
        Ok(*bonding_curve) // Simplified
    }

    /// Get or create associated token account
    async fn get_or_create_token_account(&self, mint: &Pubkey) -> Result<Pubkey> {
        // Derive ATA address using SPL library
        use spl_associated_token_account::get_associated_token_address;
        use spl_token::ID as TOKEN_PROGRAM_ID;
        
        let ata = get_associated_token_address(&self.wallet.pubkey(), mint);
        
        // Check if account exists
        match self.rpc_client.get_account_data(&ata).await {
            Ok(_) => Ok(ata),
            Err(_) => {
                // Account doesn't exist - would need to create it
                // For now, return the address (creation instruction would be added to transaction)
                Ok(ata)
            }
        }
    }

    /// Send transaction with retry logic
    async fn send_transaction_with_retry(
        &self,
        transaction: VersionedTransaction,
        max_retries: u32,
    ) -> Result<String> {
        let mut last_error = None;

        for attempt in 1..=max_retries {
            log::info!("Sending buy transaction (attempt {}/{})", attempt, max_retries);

            match self.rpc_client.send_transaction(&transaction).await {
                Ok(signature) => {
                    log::info!("Buy transaction sent: {}", signature);
                    
                    // Wait for confirmation
                    if let Err(e) = self.wait_for_confirmation(&signature).await {
                        log::warn!("Transaction sent but confirmation error: {}", e);
                    }

                    return Ok(signature.to_string());
                }
                Err(e) => {
                    log::warn!("Transaction send failed (attempt {}): {}", attempt, e);
                    last_error = Some(e);

                    if attempt < max_retries {
                        let delay = Duration::from_millis(1000 * attempt as u64);
                        log::info!("Retrying in {:?}...", delay);
                        sleep(delay).await;
                    }
                }
            }
        }

        Err(last_error
            .unwrap_or_else(|| anyhow::anyhow!("Transaction failed after {} retries", max_retries))
            .into())
    }

    /// Wait for transaction confirmation
    async fn wait_for_confirmation(
        &self,
        signature: &solana_sdk::signature::Signature,
    ) -> Result<()> {
        const MAX_WAIT_TIME: Duration = Duration::from_secs(30);
        const POLL_INTERVAL: Duration = Duration::from_millis(500);
        let start = std::time::Instant::now();

        loop {
            if start.elapsed() > MAX_WAIT_TIME {
                anyhow::bail!("Transaction confirmation timeout");
            }

            match self.rpc_client.get_signature_status(signature).await {
                Ok(Some(status)) => {
                    if status.err.is_some() {
                        anyhow::bail!("Transaction failed: {:?}", status.err);
                    }
                    if status.confirmation_status.is_some() {
                        log::info!("Transaction confirmed: {}", signature);
                        return Ok(());
                    }
                }
                Ok(None) => {
                    // Still processing
                }
                Err(e) => {
                    log::warn!("Error checking transaction status: {}", e);
                }
            }

            sleep(POLL_INTERVAL).await;
        }
    }

    /// Get wallet balance
    pub async fn get_balance(&self) -> Result<u64> {
        let balance = self
            .rpc_client
            .get_balance(&self.wallet.pubkey())
            .await
            .context("Failed to get account balance")?;

        Ok(balance)
    }
}
