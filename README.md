# YNAB Crypto Portfolio Sync 🚀

[![Release](https://img.shields.io/github/v/release/zannis/ynab-crypto-portfolio-sync-rs)](https://github.com/zannis/ynab-crypto-portfolio-sync-rs/releases)
[![Rust](https://github.com/zannis/ynab-crypto-portfolio-sync-rs/actions/workflows/build.yml/badge.svg)](https://github.com/zannis/ynab-crypto-portfolio-sync-rs/actions/workflows/build.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Automatically sync your crypto portfolio's value with [YNAB (You Need A Budget)](https://www.youneedabudget.com/). Keep
your budget up-to-date with real-time cryptocurrency valuations across multiple platforms.

## ✨ Features

- 🔄 Automatic synchronization with YNAB
- 💰 Multi-platform support:
    - Bitcoin wallets
    - EVM-compatible wallets (Ethereum, Avalanche, Polygon, zkSync, Arbitrum, Optimism)
    - Solana wallets
    - Binance exchange
- 🌐 Daily price updates
- 🔐 Secure API integration
- 🐳 Docker support

## 🚀 Quick Start

### Prerequisites

- Rust toolchain (for running locally) or Docker with Docker Compose
- YNAB Personal Access Token ([Get it here](https://app.youneedabudget.com/settings/developer))
- Exchange API keys (optional)
- Some wallet addresses to track

### Installation

```bash
# Clone the repository
git clone https://github.com/zannis/ynab-crypto-portfolio-sync-rs.git
cd ynab-crypto-portfolio-sync-rs

# Build the project
cargo build --release
```

## ⚙️ Configuration

1. Copy the environment template:

```bash
cp .env.template .env
```

2. Configure your `.env` file with the following variables:

| Variable             | Required | Description                                                      |
|----------------------|----------|------------------------------------------------------------------|
| `YNAB_ACCESS_TOKEN`  | Yes      | Your YNAB Personal Access Token                                  |
| `EVM_WALLETS`        | No       | Comma-separated list of EVM-compatible wallet addresses          |
| `BTC_WALLETS`        | No       | Comma-separated list of Bitcoin wallet addresses                 |
| `SOLANA_WALLETS`     | No       | Comma-separated list of Solana wallet addresses                  |
| `YNAB_ACCOUNT_NAME`  | No       | Custom name for your crypto tracking account (default: "Crypto") |
| `BINANCE_API_KEY`    | No       | Binance API key for exchange integration                         |
| `BINANCE_SECRET_KEY` | No       | Binance API secret                                               |

## 🔧 Usage

Once you have finished the installation, configure the environment variables in your `.env` file and then you can
either:

### Run as a Standalone Binary

```bash
cargo run
```

### Run with Docker Compose

```bash
# Build the Docker image
docker compose up
```

## 🗺️ Roadmap

- [x] Support for EVM wallets
- [x] Support for Bitcoin wallets
- [x] Binance integration
- [x] Historical tracking with daily updates
- [x] Solana support
- [ ] Support for additional crypto networks:
    - [ ] Coinbase integration
    - [ ] Algorand support
- [ ] Portfolio performance tracking
- [ ] Automated tests

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the project
2. Create your feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 💖 Support

If you find this project helpful, please consider:

- Starring the repository
- Contributing to the code
- Reporting issues or suggesting features

## 🔗 Related Projects

- [YNAB API](https://api.youneedabudget.com/)
- [Binance API](https://binance-docs.github.io/apidocs/)