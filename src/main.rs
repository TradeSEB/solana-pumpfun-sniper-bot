mod config;
mod detector;
mod instructions;
mod sniper;
mod utils;
mod wallet;

use anyhow::{Context, Result};
use tokio::signal;
use tokio_stream::StreamExt;

use config::{CliArgs, Config};
use detector::TokenDetector;
use sniper::Sniper;
use utils::init_logging;
use wallet::Wallet;

#[tokio::main]
async fn main() -> Result<()> {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // Parse CLI arguments
    let cli_args = CliArgs::parse();

    // Initialize logging
    init_logging(&cli_args.log_level)
        .context("Failed to initialize logging")?;

    log::info!("Starting Pump.fun Sniper Bot");

    // Load configuration
    let mut config = Config::from_env().context("Failed to load configuration")?;
    config.apply_cli_args(&cli_args);

    log::info!("Configuration loaded:");
    log::info!("  RPC URL: {}", config.rpc_url);
    if let Some(ref grpc_url) = config.yellowstone_grpc_url {
        log::info!("  Yellowstone gRPC URL: {}", grpc_url);
    }
    log::info!("  Buy Amount: {} SOL", config.buy_amount_sol);
    log::info!("  Priority Fee: {} micro-lamports", config.priority_fee_micro_lamports);
    log::info!("  Min Liquidity: {} SOL", config.min_initial_liquidity_sol);
    log::info!("  Dry Run: {}", config.dry_run);
    log::info!("  Jito Enabled: {}", config.jito_enabled);
    log::info!("  Blacklisted Creators: {}", config.blacklisted_creators.len());

    // Load wallet
    let wallet = Wallet::from_config(&config)
        .context("Failed to load wallet")?;

    log::info!("Wallet loaded: {}", wallet.pubkey());

    // Check balance
    let sniper = Sniper::new(
        config.rpc_url.clone(),
        wallet.clone(),
        config.clone(),
    );

    let balance = sniper.get_balance().await?;
    log::info!("Wallet balance: {:.4} SOL", balance as f64 / 1_000_000_000.0);

    if balance < 50_000_000 {
        // Less than 0.05 SOL
        log::warn!("Low balance detected! Make sure you have enough SOL for buys and fees.");
    }

    // Create token detector
    let detector = TokenDetector::new(config.clone())
        .context("Failed to create token detector")?;

    // Setup graceful shutdown
    let shutdown_signal = async {
        signal::ctrl_c()
            .await
            .expect("Failed to install CTRL+C signal handler");
        log::info!("Shutdown signal received");
    };

    // Main detection and snipe loop
    let snipe_handle = tokio::spawn(async move {
        run_snipe_loop(detector, sniper, config).await;
    });

    // Wait for shutdown signal or snipe loop completion
    tokio::select! {
        _ = shutdown_signal => {
            log::info!("Shutting down...");
            snipe_handle.abort();
        }
        _ = snipe_handle => {
            log::info!("Snipe loop completed");
        }
    }

    Ok(())
}

async fn run_snipe_loop(
    detector: TokenDetector,
    sniper: Sniper,
    config: Config,
) {
    log::info!("Starting token detection and sniping loop");

    // Start detection stream
    let mut event_stream = match detector.start_detection().await {
        Ok(stream) => stream,
        Err(e) => {
            log::error!("Failed to start detection: {}", e);
            return;
        }
    };

    log::info!("Token detection active. Waiting for new token launches...");

    // Process events from stream
    while let Some(event) = event_stream.next().await {
        log::info!(
            "New token detected: mint={}, creator={}, signature={}",
            event.mint,
            event.creator,
            event.signature
        );

        // Evaluate token against filters
        match sniper.evaluate_token(&event).await {
            Ok(should_snipe) => {
                if should_snipe {
                    log::info!("Token passed filters. Executing buy...");
                    
                    match sniper.execute_buy(&event).await {
                        Ok(tx_sig) => {
                            log::info!(
                                "Successfully sniped token {}: transaction {}",
                                event.mint,
                                tx_sig
                            );
                        }
                        Err(e) => {
                            log::error!("Failed to execute buy for {}: {}", event.mint, e);
                        }
                    }
                } else {
                    log::debug!("Token did not pass filters: {}", event.mint);
                }
            }
            Err(e) => {
                log::warn!("Error evaluating token {}: {}", event.mint, e);
            }
        }

        // Rate limiting between snipes
        utils::rate_limit_delay(config.rate_limit_ms).await;
    }

    log::warn!("Event stream ended unexpectedly");
}
