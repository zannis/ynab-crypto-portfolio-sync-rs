# ynab-crypto-portfolio-sync-rs

This is a simple Rust project whose purpose is to sync the value of EVM-based wallets with
[YNAB](https://www.youneedabudget.com/) by using [Debank](https://debank.com/).

## How it works

The app will use Debank to track the daily total value of the `WALLETS` specified in the `.env` file and update a
designated YNAB account with the total value of each wallet.

## Configuration

Use the `.env.template` file as a template to create a `.env` file with the following variables:

- `WALLETS`: A comma-separated list of wallets to track and update in YNAB.
- `YNAB_KEY`: The API key for your YNAB account. You can create
  one [here](https://app.youneedabudget.com/settings/developer).

## Running the app

You have two options to run the app:

1. Run it as a Docker container (suggested)

### Docker

1. Build the Docker image:

```bash
docker build -t ynab-crypto-portfolio-sync .
```

2. Run the Docker container:

```bash
docker run -it --rm -v $(pwd)/.env:/app/.env ynab-crypto-portfolio-sync
```

2. Run it as a standalone binary

### Standalone binary

You need to have the Rust toolchain installed on your machine. Then, run the following command:

```bash
cargo run --bin sync
```
