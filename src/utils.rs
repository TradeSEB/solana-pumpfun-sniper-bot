use anyhow::Result;
use log::LevelFilter;

/// Initialize logging based on log level string
pub fn init_logging(log_level: &str) -> Result<()> {
    let filter = match log_level.to_lowercase().as_str() {
        "trace" => LevelFilter::Trace,
        "debug" => LevelFilter::Debug,
        "info" => LevelFilter::Info,
        "warn" => LevelFilter::Warn,
        "error" => LevelFilter::Error,
        _ => LevelFilter::Info,
    };

    env_logger::Builder::from_default_env()
        .filter_level(filter)
        .format_timestamp_secs()
        .format_module_path(false)
        .init();

    Ok(())
}

/// Format lamports to SOL
pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1_000_000_000.0
}

/// Format SOL to lamports
pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1_000_000_000.0) as u64
}

/// Rate limiter helper - simple delay
pub async fn rate_limit_delay(ms: u64) {
    tokio::time::sleep(tokio::time::Duration::from_millis(ms)).await;
}

/// Estimate priority fee dynamically
/// 
/// In production, you should query get_recent_prioritization_fees
pub async fn estimate_priority_fee(
    _rpc_client: &solana_client::nonblocking::rpc_client::RpcClient,
    base_fee_micro_lamports: u64,
) -> u64 {
    // Simple heuristic - in production, query recent fees
    // For now, return base fee with some jitter
    use rand::Rng;
    let mut rng = rand::thread_rng();
    let jitter = rng.gen_range(0..50_000);
    base_fee_micro_lamports + jitter
}

/// Check if a string looks like spam (simple heuristic)
pub fn is_spam_name(name: &str) -> bool {
    // Simple checks - can be enhanced
    name.len() > 50
        || name.chars().filter(|c| c.is_alphanumeric()).count() < 3
        || name.matches("test").count() > 2
        || name.matches("spam").count() > 0
}

/// Check if a symbol looks like spam
pub fn is_spam_symbol(symbol: &str) -> bool {
    symbol.len() > 10 || symbol.chars().any(|c| !c.is_alphanumeric())
}
