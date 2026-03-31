# Recleans - Plastic Bottle Collection dApp

Recleans is a decentralized application (dApp) on the Stellar Soroban Testnet that allows users to **deposit plastic bottles**, track total deposits, and **redeem rewards**.

---

## Features

- Deposit plastic bottles into the system
- Track total bottles deposited per user
- Redeem collected bottles
- Fully deployed on Stellar Soroban Testnet

---

## Setup

### Prerequisites

- Install [Stellar CLI](https://github.com/stellar/stellar-cli)
- Install Rust toolchain (`rustup`, `cargo`, `rustc`)
- Target `wasm32v1-none` installed:

```bash
rustup target add wasm32v1-none
```

### Clone Repo

```bash
git clone https://github.com/tsabit-dv/stellar-workshop.git
cd stellar-workshop
```

### Building the Contract

```bash
stellar contract build
```

WASM file will be generated at:

```
target/wasm32v1-none/release/hello_world.wasm
```

### Deploying to Testnet

1. Generate or use existing key:

```bash
stellar keys generate tsabit
```

2. Fund account via Friendbot:

```bash
curl "https://friendbot.stellar.org?addr=<PUBLIC_KEY_HERE>"
```

3. Deploy contract:

```bash
stellar contract deploy \
  --wasm target/wasm32v1-none/release/hello_world.wasm \
  --network testnet \
  --source tsabit
```

4. Save the Contract ID for usage and README:

```
CAJ6T7AIKEC55OKAAPT4OB2S3H4TV2Z73JKJJT2PADLLKBR6TI526QKX
```

---

## Testnet Deployment Info

| Info | Value |
|------|-------|
| **Contract ID** | `CAJ6T7AIKEC55OKAAPT4OB2S3H4TV2Z73JKJJT2PADLLKBR6TI526QKX` |
| **Deployer Account (tsabit)** | `GCF7V6F6NKOMJVKTOST43IKKJUE6QX7YSHOT54JZARCIIV3SSHE2IWW2` |
| **Transaction** | [View on Stellar Expert](https://stellar.expert) |
| **Contract Explorer** | [Soroban Lab](https://lab.stellar.org) |

---

## Usage

Once deployed, the following functions are available:

### Deposit bottles

```bash
stellar contract invoke \
  --network testnet \
  --source tsabit \
  --id CAJ6T7AIKEC55OKAAPT4OB2S3H4TV2Z73JKJJT2PADLLKBR6TI526QKX \
  --fn deposit \
  --arg <number_of_bottles>
```

### Get total bottles deposited by user

```bash
stellar contract invoke \
  --network testnet \
  --source tsabit \
  --id CAJ6T7AIKEC55OKAAPT4OB2S3H4TV2Z73JKJJT2PADLLKBR6TI526QKX \
  --fn get_total \
  --arg <user_address>
```

### Redeem bottles

```bash
stellar contract invoke \
  --network testnet \
  --source tsabit \
  --id CAJ6T7AIKEC55OKAAPT4OB2S3H4TV2Z73JKJJT2PADLLKBR6TI526QKX \
  --fn redeem \
  --arg <user_address>
```

---

## Notes

- This dApp is fully on testnet, no real assets involved.
- Replace `<PUBLIC_KEY_HERE>` and `<user_address>` with the actual Stellar account keys.
- Designed to simulate a recycling reward system.