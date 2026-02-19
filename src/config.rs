use serde::{Deserialize, Serialize};
use std::str::FromStr;

/// Pump.fun program ID
pub const PUMPFUN_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

/// Create instruction discriminator for Pump.fun
pub const CREATE_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];

/// Main configuration for the sniper bot
#[derive(Debug, Clone)]
pub struct Config {
    /// Solana RPC endpoint URL
    pub rpc_url: String,
    /// Yellowstone Geyser gRPC endpoint (optional)
    pub yellowstone_grpc_url: Option<String>,
    /// Wallet private key (base58 encoded)
    pub private_key: Option<String>,
    /// Wallet mnemonic phrase (alternative to private_key)
    pub mnemonic: Option<String>,
    /// Buy amount in SOL
    pub buy_amount_sol: f64,
    /// Priority fee in micro-lamports
    pub priority_fee_micro_lamports: u64,
    /// Minimum initial liquidity (in SOL) to snipe
    pub min_initial_liquidity_sol: f64,
    /// Maximum initial liquidity (in SOL) to snipe
    pub max_initial_liquidity_sol: Option<f64>,
    /// Blacklist of creator wallet addresses to avoid
    pub blacklisted_creators: Vec<String>,
    /// Enable dry-run mode (simulate without executing)
    pub dry_run: bool,
    /// Enable Jito bundle support
    pub jito_enabled: bool,
    /// Jito tip amount in lamports
    pub jito_tip_lamports: u64,
    /// Jito block engine URL
    pub jito_block_engine_url: Option<String>,
    /// Maximum compute units for transactions
    pub max_compute_units: u32,
    /// Slippage tolerance in basis points
    pub slippage_bps: u16,
    /// Use WebSocket fallback if gRPC unavailable
    pub use_websocket_fallback: bool,
    /// Rate limit delay between RPC calls (ms)
    pub rate_limit_ms: u64,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            rpc_url: "https://api.mainnet-beta.solana.com".to_string(),
            yellowstone_grpc_url: None,
            private_key: None,
            mnemonic: None,
            buy_amount_sol: 0.1,
            priority_fee_micro_lamports: 100_000, // 0.0001 SOL
            min_initial_liquidity_sol: 0.0,
            max_initial_liquidity_sol: None,
            blacklisted_creators: vec![],
            dry_run: true,
            jito_enabled: false,
            jito_tip_lamports: 10_000,
            jito_block_engine_url: None,
            max_compute_units: 1_400_000,
            slippage_bps: 50,
            use_websocket_fallback: true,
            rate_limit_ms: 100,
        }
    }
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> anyhow::Result<Self> {
        let mut config = Self::default();

        if let Ok(rpc_url) = std::env::var("RPC_URL") {
            config.rpc_url = rpc_url;
        }

        if let Ok(grpc_url) = std::env::var("YELLOWSTONE_GRPC_URL") {
            config.yellowstone_grpc_url = Some(grpc_url);
        }

        if let Ok(private_key) = std::env::var("PRIVATE_KEY_BASE58") {
            config.private_key = Some(private_key);
        }

        if let Ok(mnemonic) = std::env::var("MNEMONIC") {
            config.mnemonic = Some(mnemonic);
        }

        if let Ok(buy_amount) = std::env::var("BUY_AMOUNT_SOL") {
            config.buy_amount_sol = f64::from_str(&buy_amount)
                .map_err(|e| anyhow::anyhow!("Invalid BUY_AMOUNT_SOL: {}", e))?;
        }

        if let Ok(priority_fee) = std::env::var("PRIORITY_FEE_MICRO_LAMPORTS") {
            config.priority_fee_micro_lamports = u64::from_str(&priority_fee)
                .map_err(|e| anyhow::anyhow!("Invalid PRIORITY_FEE_MICRO_LAMPORTS: {}", e))?;
        }

        if let Ok(min_liq) = std::env::var("MIN_INITIAL_LIQUIDITY_SOL") {
            config.min_initial_liquidity_sol = f64::from_str(&min_liq)
                .map_err(|e| anyhow::anyhow!("Invalid MIN_INITIAL_LIQUIDITY_SOL: {}", e))?;
        }

        if let Ok(max_liq) = std::env::var("MAX_INITIAL_LIQUIDITY_SOL") {
            config.max_initial_liquidity_sol = Some(f64::from_str(&max_liq)
                .map_err(|e| anyhow::anyhow!("Invalid MAX_INITIAL_LIQUIDITY_SOL: {}", e))?);
        }

        if let Ok(blacklist) = std::env::var("BLACKLISTED_CREATORS") {
            config.blacklisted_creators = blacklist
                .split(',')
                .map(|s| s.trim().to_string())
                .filter(|s| !s.is_empty())
                .collect();
        }

        if let Ok(dry_run) = std::env::var("DRY_RUN") {
            config.dry_run = dry_run.to_lowercase() == "true" || dry_run == "1";
        }

        if let Ok(jito_enabled) = std::env::var("JITO_ENABLED") {
            config.jito_enabled = jito_enabled.to_lowercase() == "true" || jito_enabled == "1";
        }

        if let Ok(tip) = std::env::var("JITO_TIP_LAMPORTS") {
            config.jito_tip_lamports = u64::from_str(&tip)
                .map_err(|e| anyhow::anyhow!("Invalid JITO_TIP_LAMPORTS: {}", e))?;
        }

        if let Ok(jito_url) = std::env::var("JITO_BLOCK_ENGINE_URL") {
            config.jito_block_engine_url = Some(jito_url);
        }

        if let Ok(compute_units) = std::env::var("MAX_COMPUTE_UNITS") {
            config.max_compute_units = u32::from_str(&compute_units)
                .map_err(|e| anyhow::anyhow!("Invalid MAX_COMPUTE_UNITS: {}", e))?;
        }

        if let Ok(slippage) = std::env::var("SLIPPAGE_BPS") {
            config.slippage_bps = u16::from_str(&slippage)
                .map_err(|e| anyhow::anyhow!("Invalid SLIPPAGE_BPS: {}", e))?;
        }

        if let Ok(use_ws) = std::env::var("USE_WEBSOCKET_FALLBACK") {
            config.use_websocket_fallback = use_ws.to_lowercase() == "true" || use_ws == "1";
        }

        if let Ok(rate_limit) = std::env::var("RATE_LIMIT_MS") {
            config.rate_limit_ms = u64::from_str(&rate_limit)
                .map_err(|e| anyhow::anyhow!("Invalid RATE_LIMIT_MS: {}", e))?;
        }

        Ok(config)
    }

    /// Apply CLI arguments to override config
    pub fn apply_cli_args(&mut self, args: &CliArgs) {
        if let Some(rpc_url) = &args.rpc_url {
            self.rpc_url = rpc_url.clone();
        }

        if let Some(grpc_url) = &args.yellowstone_grpc_url {
            self.yellowstone_grpc_url = Some(grpc_url.clone());
        }

        if let Some(buy_amount) = args.buy_amount {
            self.buy_amount_sol = buy_amount;
        }

        if let Some(priority_fee) = args.priority_fee {
            self.priority_fee_micro_lamports = priority_fee;
        }

        if let Some(min_liq) = args.min_liquidity {
            self.min_initial_liquidity_sol = min_liq;
        }

        if let Some(max_liq) = args.max_liquidity {
            self.max_initial_liquidity_sol = Some(max_liq);
        }

        if !args.blacklist.is_empty() {
            self.blacklisted_creators = args.blacklist.clone();
        }

        if args.dry_run {
            self.dry_run = true;
        }

        if args.jito_bundle {
            self.jito_enabled = true;
        }
    }
}

/// CLI arguments structure
#[derive(Debug, Clone, clap::Parser)]
#[command(name = "pumpfun-sniper")]
#[command(about = "A high-performance Solana sniper bot for Pump.fun token launches")]
pub struct CliArgs {
    /// Solana RPC endpoint URL
    #[arg(long, env = "RPC_URL")]
    pub rpc_url: Option<String>,

    /// Yellowstone Geyser gRPC endpoint
    #[arg(long, env = "YELLOWSTONE_GRPC_URL")]
    pub yellowstone_grpc_url: Option<String>,

    /// Buy amount in SOL
    #[arg(long)]
    pub buy_amount: Option<f64>,

    /// Priority fee in micro-lamports
    #[arg(long)]
    pub priority_fee: Option<u64>,

    /// Minimum initial liquidity in SOL
    #[arg(long)]
    pub min_liquidity: Option<f64>,

    /// Maximum initial liquidity in SOL
    #[arg(long)]
    pub max_liquidity: Option<f64>,

    /// Blacklisted creator addresses (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub blacklist: Vec<String>,

    /// Enable dry-run mode
    #[arg(long)]
    pub dry_run: bool,

    /// Enable Jito bundle support
    #[arg(long)]
    pub jito_bundle: bool,

    /// Log level (trace, debug, info, warn, error)
    #[arg(long, default_value = "info")]
    pub log_level: String,
}
