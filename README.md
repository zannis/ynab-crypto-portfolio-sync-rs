# YNAB Crypto Portfolio Sync 🚀

[![Rust](https://github.com/zannis/ynab-crypto-portfolio-sync-rs/actions/workflows/rust.yml/badge.svg)](https://github.com/zannis/ynab-crypto-portfolio-sync-rs/actions/workflows/rust.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

Automatically sync your crypto portfolio's value with [YNAB (You Need A Budget)](https://www.youneedabudget.com/). Keep
your budget up-to-date with real-time cryptocurrency valuations across multiple platforms.

## ✨ Features

- 🔄 Automatic synchronization with YNAB
- 💰 Multi-platform support:
    - Bitcoin wallets
    - EVM-compatible wallets (Ethereum, Avalanche, Polygon, zkSync, Arbitrum, Optimism)
    - Binance exchange
- 🌐 Real-time price updates
- 🔐 Secure API integration
- 🐳 Docker support

## 🚀 Quick Start

### Prerequisites

- Rust toolchain (for running locally) or Docker with Docker Compose
- YNAB API key ([Get it here](https://app.youneedabudget.com/settings/developer))
- Wallet addresses or exchange API keys

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
| `YNAB_KEY`           | Yes      | Your YNAB API key                                                |
| `EVM_WALLETS`        | No       | Comma-separated list of EVM-compatible wallet addresses          |
| `BTC_WALLETS`        | No       | Comma-separated list of Bitcoin wallet addresses                 |
| `YNAB_ACCOUNT_NAME`  | No       | Custom name for your crypto tracking account (default: "Crypto") |
| `BINANCE_API_KEY`    | No       | Binance API key for exchange integration                         |
| `BINANCE_SECRET_KEY` | No       | Binance API secret                                               |

## 🔧 Usage

Make sure you have configured the environment variables in `.env` file and then:

### Running as a Standalone Binary

```bash
cargo run
```

### Running with Docker Compose

```bash
# Build the Docker image
docker compose up
```

## 🗺️ Roadmap

- [ ] Historical tracking with daily updates
- [ ] Support for additional crypto networks:
    - [ ] Coinbase integration
    - [ ] Solana support
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

```