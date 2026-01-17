# 🏭 ICRC Factory (`icrc-factory`)

A canister that **stores ICRC ledger/index WASMs**
and lets callers **create and manage ICRC ledger suite canisters** (ledger + index) on the Internet Computer.

<div align="center" style="display:flex;flex-direction:column;">

[![Internet Computer portal](https://img.shields.io/badge/Internet-Computer-grey?logo=internet%20computer)](https://internetcomputer.org)
[![GitHub CI Checks Workflow Status](https://img.shields.io/github/actions/workflow/status/AntonioVentilii/icrc-factory/checks.yml?logo=github&label=Checks)](https://github.com/AntonioVentilii/icrc-factory/actions/workflows/checks.yml)
[![GitHub CI Tests Workflow Status](https://img.shields.io/github/actions/workflow/status/AntonioVentilii/icrc-factory/tests.yml?logo=github&label=Tests)](https://github.com/AntonioVentilii/icrc-factory/actions/workflows/tests.yml)

</div>

> **What this gives you**
>
> - Create **new** ledger canisters and index canisters
> - Update existing ledger configuration (set name, symbol, index canister)

---

## 📚 Table of Contents

- [🧭 Overview](#overview)
- [🏛️ Architecture](#architecture)
- [🧾 Public API](#public-api)
  - [Controller-only Updates](#controller-only-updates)
  - [User Updates](#user-updates)
- [🚀 Getting Started](#getting-started)
  - [Prerequisites](#prerequisites)
  - [Local Development](#local-development)
  - [Deploy to Staging / Mainnet](#deploy-to-staging--mainnet)
- [🧪 Examples](#examples)
  - [Upload WASM from URL](#upload-wasm-from-url)
  - [Create a Ledger](#create-a-ledger)
  - [Create an Index](#create-an-index)
  - [Set Token Name / Symbol](#set-token-name--symbol)
  - [Attach an Index to a Ledger](#attach-an-index-to-a-ledger)
- [💳 Payment Handling](#payment-handling)
- [🙏 Credits and References](#credits-and-references)

---

## 🧭 Overview

**ICRC Factory** is a canister that acts as an “ICRC ledger suite factory”.

At a high level it:

1. **Stores** the WASM binaries used to install:
   - an ICRC ledger canister (e.g. `ic-icrc1-ledger.wasm.gz`)
   - an ICRC index canister (e.g. `ic-icrc1-index-ng.wasm.gz`)
2. **Creates new canisters** via the management canister
3. **Installs** the stored WASM with init args
4. Charges a **fee** for creation actions through a payment guard

---

## 🏛️ Architecture

- **Factory Canister (`icrc-factory`)**
  - Stores WASMs (ledger and index)
  - Exposes methods to create ledgers/indexes and to update ledgers

- **Ledger Canister (ICRC-1)**
  - Created by the factory
  - Can be upgraded by the factory to update config such as name/symbol/index

- **Index Canister (ICRC-1 index-ng)**
  - Created by the factory
  - Points to an existing ledger (`ledger_id`)

---

## 🧾 Public API

### Queries

### Controller-only Updates

These are restricted to **canister controllers**.

#### `set_ledger_wasm(wasm: Vec<u8>)`

Stores a new ledger WASM.

#### `set_ledger_wasm_from_url(url: String) -> SetWasmResult`

Fetches the ledger WASM from a URL and stores it.

#### `set_index_wasm(wasm: Vec<u8>)`

Stores a new index WASM.

#### `set_index_wasm_from_url(url: String) -> SetWasmResult`

Fetches the index WASM from a URL and stores it.

> [!NOTE]
> The project includes an HTTP response transform (`transform_wasm_response`) to sanitise fetched WASM responses.

---

### User Updates

These require the caller to be **non-anonymous**.

#### `create_icrc_ledger(args: CreateIcrcLedgerArgs, payment: Option<PaymentType>) -> CreateCanisterResult`

Creates a new ICRC ledger canister.

**What the caller must provide**

- `args`: a `CreateIcrcLedgerArgs` struct (all fields optional)
- `payment`: optional `PaymentType` (defaults to `AttachedCycles` if `None`)

**`CreateIcrcLedgerArgs` fields**

- `symbol: Option<String>` – optional token symbol
- `name: Option<String>` – optional token name
- `transfer_fee: Option<u64>` – optional transfer fee (smallest unit)
- `decimals: Option<u8>` – optional decimals
- `minting_account: Option<Account>` – optional minting account

Any omitted fields fall back to the ledger’s defaults.

---

#### `create_icrc_index(args: CreateIcrcIndexArgs, payment: Option<PaymentType>) -> CreateCanisterResult`

Creates a new ICRC index canister.

**What the caller must provide**

- `args.ledger_id: Principal` – required
- `payment`: optional `PaymentType` (defaults to `AttachedCycles`)

---

#### `set_index_canister(args: SetIndexCanisterArgs) -> SetCanisterResult`

Associates an index canister with a ledger by upgrading the ledger configuration.

**What the caller must provide**

- `args.ledger_id: Principal` – required
- `args.index_id: Principal` – required

---

#### `set_symbol(args: SetSymbolArgs) -> SetCanisterResult`

Updates a ledger’s token symbol by upgrading the ledger configuration.

**What the caller must provide**

- `args.ledger_id: Principal` – required
- `args.symbol: String` – required

---

#### `set_name(args: SetNameArgs) -> SetCanisterResult`

Updates a ledger’s token name by upgrading the ledger configuration.

**What the caller must provide**

- `args.ledger_id: Principal` – required
- `args.name: String` – required

---

## 🚀 Getting Started

### Prerequisites

- `dfx` installed and configured
- Node.js + npm (for formatting / scripts)

---

### Local Development

```bash
# install JS tooling (formatting, etc.)
npm install

# format all sources (prettier + rust formatting script)
npm run format

# lint all sources
npm run lint
```

[//]: # 'TODO: add local deploy instructions if you have `dfx.json` configured for local.'

---

### Deploy to Staging / Mainnet

```bash
# Deploy to staging
npm run deploy:staging

# Deploy to mainnet
npm run deploy:prod
```

If you need to reinstall on mainnet (destructive):

```bash
npm run reinstall:prod
```

---

## 🧪 Examples

The commands below are based on your scripts. Adjust cycle amounts and arguments as needed.

### Upload WASM from URL

```bash
# Set ledger wasm (prod)
npm run wasm:ledger:prod

# Set index wasm (prod)
npm run wasm:index:prod

# Or both
npm run wasm:prod
```

---

### Create a Ledger

Your script shows attaching cycles and calling `create_icrc_ledger`:

```bash
npm run ledger:prod
```

To pass arguments explicitly (example template):

```bash
dfx canister call icrc-factory --ic \
  --wallet "$(dfx identity get-wallet --ic)" \
  --with-cycles 1_100_000_000_000 \
  create_icrc_ledger \
  '(record {
      symbol = opt "TKN";
      name = opt "My Token";
      transfer_fee = opt 10_000;
      decimals = opt 8;
      minting_account = null;
    }, null)'
```

---

### Create an Index

```bash
npm run index:prod
```

Explicit example:

```bash
dfx canister call icrc-factory --ic \
  --wallet "$(dfx identity get-wallet --ic)" \
  --with-cycles 500_000_000_000 \
  create_icrc_index \
  '(record { ledger_id = principal "aaaaa-aa"; }, null)'
```

---

### Set Token Name / Symbol

```bash
dfx canister call icrc-factory --ic set_symbol \
  '(record { ledger_id = principal "aaaaa-aa"; symbol = "TKN"; })'

dfx canister call icrc-factory --ic set_name \
  '(record { ledger_id = principal "aaaaa-aa"; name = "My Token"; })'
```

---

### Attach an Index to a Ledger

```bash
dfx canister call icrc-factory --ic set_index_canister \
  '(record { ledger_id = principal "aaaaa-aa"; index_id = principal "bbbbb-bb"; })'
```

---

## 💳 Payment Handling

TBD

[//]: # 'TODO: add details on how payments are handled, what `PaymentType` options exist, and how to top up cycles. Plus the costs of each method'

---

## 🙏 Credits and References

The code was inspired by a series of sources and projects, including:

- [Juno](https://github.com/junobuild/juno)'s CMC and Orbiter factory implementations.
- [Chain Fusion Signer](https://github.com/dfinity/chain-fusion-signer)'s payment guard patterns through [PAPI](https://github.com/dfinity/papi).
- Ledger and Index Canisters types from the [ICRC Ledger Suite](https://github.com/dfinity/ic/tree/master/rs/ledger_suite) specification.
