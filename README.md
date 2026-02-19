# Pump.fun Sniper Bot (Rust)

![Rust](https://img.shields.io/badge/rust-1.70+-orange.svg)
![Solana](https://img.shields.io/badge/solana-1.18-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

A high-performance, production-grade Solana sniper bot that detects brand-new token launches on Pump.fun in real-time and automatically executes buys when tokens match configurable filters.

## Features

- 🚀 **Real-time Detection**: Uses Yellowstone Geyser gRPC streaming (preferred) or WebSocket fallback to detect new token launches instantly
- 🎯 **Fast Execution**: Automatically executes buy transactions when tokens pass filters
- 🔍 **Configurable Filters**: 
  - Minimum/maximum initial liquidity thresholds
  - Creator blacklist
  - Token metadata validation (name/symbol spam detection)
- 💰 **Smart Fee Management**: Dynamic priority fee estimation with configurable multipliers
- 🛡️ **Safety Features**:
  - Dry-run mode for testing without executing transactions
  - Slippage protection
  - Rate limiting to avoid RPC bans
  - Transaction retry logic with exponential backoff
  - Graceful shutdown on Ctrl+C
- 🔐 **Secure Wallet Management**: Supports both base58 private keys and BIP39 mnemonics
- 📊 **Comprehensive Logging**: Detailed logging at multiple levels
- ⚡ **High Performance**: Built with Tokio async runtime for maximum speed
- 🎁 **Optional Jito Support**: MEV protection via Jito bundles (toggleable)

## Tech Stack

- **Language**: Rust (edition 2021)
- **Async Runtime**: Tokio
- **Blockchain**: Solana SDK 1.18
- **Streaming**: Yellowstone Geyser gRPC (with WebSocket fallback)
- **HTTP Client**: Reqwest
- **CLI**: Clap
- **Logging**: env_logger

## Installation

### Prerequisites

- Rust 1.70 or later ([Install Rust](https://www.rust-lang.org/tools/install))
- A Solana wallet with SOL for buys and transaction fees
- A reliable Solana RPC endpoint (recommended: Helius, QuickNode, or similar)
- (Optional) Yellowstone Geyser gRPC endpoint for real-time streaming

### Build from Source

```bash
# Clone the repository
git clone https://github.com/yourusername/pumpfun-sniper-bot.git
cd pumpfun-sniper-bot

# Build in release mode (optimized for performance)
cargo build --release

# The binary will be at: target/release/pumpfun-sniper
```

## Configuration

### Environment Variables

Create a `.env` file in the project root (see `.env.example` for template):

```bash
# Required: Solana RPC endpoint
RPC_URL=https://api.mainnet-beta.solana.com

# Optional: Yellowstone Geyser gRPC endpoint (for real-time streaming)
YELLOWSTONE_GRPC_URL=grpc://your-endpoint:10000

# Required: Wallet (choose one)
PRIVATE_KEY_BASE58=your_base58_private_key_here
# OR
MNEMONIC=word1 word2 word3 ... word12

# Trading Configuration
BUY_AMOUNT_SOL=0.1                    # Amount in SOL to buy per token
PRIORITY_FEE_MICRO_LAMPORTS=100000    # Priority fee (0.0001 SOL)

# Filter Configuration
MIN_INITIAL_LIQUIDITY_SOL=0.0         # Minimum liquidity to snipe
MAX_INITIAL_LIQUIDITY_SOL=            # Maximum liquidity (empty = no limit)
BLACKLISTED_CREATORS=                 # Comma-separated creator addresses to avoid

# Execution Mode
DRY_RUN=true                          # Set to false to execute real transactions

# Jito Configuration (optional)
JITO_ENABLED=false
JITO_TIP_LAMPORTS=10000
JITO_BLOCK_ENGINE_URL=https://mainnet.block-engine.jito.wtf

# Transaction Configuration
MAX_COMPUTE_UNITS=1400000
SLIPPAGE_BPS=50                       # Slippage tolerance (50 = 0.5%)

# Detection Configuration
USE_WEBSOCKET_FALLBACK=true           # Use WebSocket if gRPC unavailable
RATE_LIMIT_MS=100                     # Delay between RPC calls
```

### CLI Arguments

You can override environment variables using CLI arguments:

```bash
# Run with custom buy amount
./target/release/pumpfun-sniper --buy-amount 0.5

# Enable dry-run mode
./target/release/pumpfun-sniper --dry-run

# Set custom priority fee
./target/release/pumpfun-sniper --priority-fee 200000

# Set minimum liquidity filter
./target/release/pumpfun-sniper --min-liquidity 0.1

# Blacklist specific creators
./target/release/pumpfun-sniper --blacklist ADDRESS1,ADDRESS2

# Enable Jito bundles
./target/release/pumpfun-sniper --jito-bundle

# Set log level
./target/release/pumpfun-sniper --log-level debug
```

## Usage

### Basic Usage

1. **Set up your `.env` file** with your RPC URL and wallet credentials
2. **Start with dry-run mode** to test:
   ```bash
   ./target/release/pumpfun-sniper --dry-run
   ```
3. **Monitor the logs** to see token detections and filter evaluations
4. **Once confident, disable dry-run**:
   ```bash
   ./target/release/pumpfun-sniper
   ```

### Example Output

```
[INFO] Starting Pump.fun Sniper Bot
[INFO] Configuration loaded:
[INFO]   RPC URL: https://api.mainnet-beta.solana.com
[INFO]   Buy Amount: 0.1 SOL
[INFO]   Priority Fee: 100000 micro-lamports
[INFO]   Dry Run: true
[INFO] Wallet loaded: 7xKXtg2CW87d97TXJSDpbD5jBkheTqA83TZRuJosgAsU
[INFO] Wallet balance: 1.5000 SOL
[INFO] Starting token detection and sniping loop
[INFO] Token detection active. Waiting for new token launches...
[INFO] New token detected: mint=ABC123..., creator=XYZ789..., signature=...
[INFO] Evaluating new token: mint=ABC123..., creator=XYZ789...
[INFO] Token passed all filters: ABC123...
[INFO] [DRY RUN] Would buy token: mint=ABC123..., amount=0.1 SOL
```

## Architecture

### Project Structure

```
pumpfun-sniper-bot/
├── src/
│   ├── main.rs          # CLI entrypoint and main loop
│   ├── config.rs        # Configuration management
│   ├── wallet.rs        # Wallet/keypair loading
│   ├── detector.rs      # Real-time token detection (Geyser/WebSocket)
│   ├── sniper.rs        # Filter evaluation and buy execution
│   ├── instructions.rs  # Pump.fun instruction builders
│   └── utils.rs         # Helper functions
├── Cargo.toml           # Dependencies and project metadata
├── .env.example         # Environment variable template
├── .gitignore          # Git ignore rules
└── README.md           # This file
```

### How It Works

1. **Token Detection**:
   - **Preferred**: Yellowstone Geyser gRPC stream subscribes to Pump.fun program transactions
   - **Fallback**: WebSocket/RPC polling for new transactions
   - Filters for Create instruction (discriminator: `[24, 30, 200, 40, 5, 28, 7, 119]`)
   - Extracts: mint address, bonding curve, creator wallet

2. **Filter Evaluation**:
   - Checks creator blacklist
   - Validates initial liquidity (if available)
   - Checks token metadata for spam patterns
   - Applies custom filters

3. **Buy Execution** (if filters pass):
   - Builds Pump.fun buy instruction
   - Calculates minimum tokens out with slippage
   - Adds priority fees and compute unit limits
   - Signs transaction with wallet
   - Sends with retry logic
   - Waits for confirmation

4. **Rate Limiting**: Implements delays between operations to avoid RPC bans

### Pump.fun Program Details

- **Program ID**: `6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P`
- **Create Instruction Discriminator**: `[24, 30, 200, 40, 5, 28, 7, 119]`
- **Detection Method**: Real-time transaction monitoring via Geyser gRPC or WebSocket

## Safety Considerations

1. **ALWAYS test in dry-run mode first**
2. **Start with small buy amounts** (0.01-0.1 SOL)
3. **Use a dedicated wallet** with limited funds
4. **Monitor logs closely** during initial runs
5. **Set appropriate filters** to avoid obvious scams
6. **Use reliable RPC endpoints** to avoid transaction failures
7. **Be aware of gas fees** - they can eat into profits
8. **Memecoins are extremely risky** - you can lose everything
9. **Test on devnet first** before using mainnet

## Troubleshooting

### Common Issues

**"Failed to load wallet"**
- Ensure `PRIVATE_KEY_BASE58` or `MNEMONIC` is set correctly
- Check that private key is valid base58 encoding
- Verify mnemonic phrase is 12 or 24 words

**"Failed to connect to Yellowstone Geyser"**
- Check your gRPC endpoint URL
- Verify network connectivity
- Bot will fall back to WebSocket if configured

**"No tokens detected"**
- This is normal - new token launches are infrequent
- Check that RPC endpoint is working
- Verify Pump.fun program ID is correct
- Try increasing log level to debug

**"Transaction failed"**
- Ensure sufficient SOL balance for buys and fees
- Check RPC endpoint is working
- Verify slippage tolerance is appropriate
- Try increasing priority fee
- Check that bonding curve account exists

**"Token detection stopped"**
- Check network connection
- Verify RPC/gRPC endpoint is accessible
- Check logs for errors
- Restart the bot

## Development

### Running Tests

```bash
cargo test
```

### Building for Development

```bash
cargo build
```

### Code Formatting

```bash
cargo fmt
```

### Linting

```bash
cargo clippy
```

## Support

- Telegram: https://t.me/trade_SEB
- Twitter: https://x.com/TradeSEB_

