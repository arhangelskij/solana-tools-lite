# solana-tools-lite
[![Crates.io](https://img.shields.io/crates/v/solana-tools-lite.svg)](https://crates.io/crates/solana-tools-lite)
[![Docs.rs](https://docs.rs/solana-tools-lite/badge.svg)](https://docs.rs/solana-tools-lite)
[![e2e Create + Sign + Verify V0 + send to Devnet](https://github.com/arhangelskij/solana-tools-lite/actions/workflows/e2e-devnet-v0.yml/badge.svg)](https://github.com/arhangelskij/solana-tools-lite/actions/workflows/e2e-devnet-v0.yml)

🛠 A lightweight, SDK-free, offline-first signing toolkit for Solana keys, messages, and transactions — with CI-friendly JSON output.


## ⚡ Performance & Build Efficiency

Solana-tools-lite is engineered to be lightweight from the ground up. By eliminating the heavy dependency tree of the official Solana SDK, it achieves drastic improvements in build times and resource usage.

The difference is most critical in CI/CD pipelines, Docker builds, and ephemeral environments where "cold builds" are frequent.

| Metric | solana-tools-lite | solana-cli (Baseline) | Improvement |
| :--- | :--- | :--- | :--- |
| Scope | Full CLI + Core Library | Empty Project (Deps Only) | — |
| Cold Build Time | ~14.7s | ~1m 17s | ~5.2x Faster |
| Dependencies | ~85 | ~729 | ~8.6x Fewer |
| CPU Time (Cost) | ~34.5s | ~5m 03s | ~8.8x Lighter |

**Micro-benchmarks:** Core Ed25519 signing is ~13.1µs (single-thread) in our bench, on par with the Solana SDK.

> **Note:** The baseline benchmark measures the *minimum baseline cost* to compile dependencies for a `solana-cli` empty project (empty `main`). Solana-tools-lite compiles the entire functional tool suite in a fraction of that time.
>
> *Benchmarks run on release profile with LTO enabled.*

## Why?

The official Solana CLI is a powerful all-in-one toolbox — great for nodes and RPC workflows, but overkill for cold wallets and offline signing.

`solana-tools-lite` is a minimal, offline-first toolkit for key management and signing:
- **Offline by design:** no RPC client, no networking code — suited for air-gapped workflows.
- **Smaller attack surface:** fewer dependencies, easier to audit, less supply-chain risk.
- **Fast builds & CI:** predictable build times for ephemeral runners and reproducible pipelines.
- **Safety-first UX:** explicit warnings for partial context (e.g., V0 lookups without tables) instead of guessing.

## Features

- **Key Management:** Generate keypairs from BIP-39 mnemonics (supports passphrases).
- **Transaction Signing:** Full support for Legacy + V0 (Versioned) transactions.
- **Flexible Input:** Accepts transactions in JSON, Base64, or Base58 formats.
- **Message Signing:** Sign and verify arbitrary messages (fully offline).
- **V0 Context:** Optional ALT (Address Lookup Table) resolution via `--tables` for safer offline analysis.
- **CI/CD Ready:** Pipeline-friendly flags (`--yes`, `--max-fee`) and structured JSON output.

## 📦 Installation

### From Crates.io

```bash
cargo install solana-tools-lite-cli
```

After install, you can use either:
`solana-tools-lite` or `stl`.

### Rust library (core)

If you want to use the core library in your own Rust code:

```bash
cargo add solana-tools-lite
```

### From source

```bash
git clone https://github.com/arhangelskij/solana-tools-lite
cd solana-tools-lite
cargo install --path solana-tools-lite-cli
```

## 🚀 Usage

### 1. Generate a new wallet
Create a keypair and save a wallet JSON (includes the BIP-39 mnemonic). By default it prints only the public key; use `--unsafe-show-secret` if you intentionally want to print secrets.

```bash
solana-tools-lite gen --output ./keys.json
```

### 2. Sign a transaction (Offline / Pipeline)
Read an unsigned transaction from a file (or stdin), sign it, and save the result.

```bash
# Read from file, write to file
solana-tools-lite sign-tx --input unsigned.json --keypair wallet.json --output signed.json

# Pipeline mode (read from stdin, write to file). Use --yes because stdin is consumed by --input -
cat unsigned.json | solana-tools-lite sign-tx --input - --keypair wallet.json --output signed.json --yes
```

### 3. Verify a signature
Check if a signature is valid for a specific message.

```bash
solana-tools-lite verify --pubkey <PUBKEY> --message "Verify me" --signature <SIGNATURE>
```

### 4. V0 transaction signing + summary (safe mode)
Sign a Versioned Transaction and emit a summary using an offline ALT context.

```bash
solana-tools-lite sign-tx --input unsigned_v0.b64 --keypair kp.json --tables tables.json --output signed_v0.b64 --summary-json
```

### 5. Analyze a transaction
Inspect a transaction without signing. Useful for verifying fees, transfers, and privacy impact before approval.

```bash
solana-tools-lite analyze --input tx.b64 --tables tables.json
```

### 🤖 Scripting & CI Integration
Combine with `jq` for reliable one-liners:

```bash
# Extract signature only
solana-tools-lite sign --keypair wallet.json --message "Auth" --json | jq -r '.signature_base58'

# Verify exits non-zero on invalid signatures
solana-tools-lite verify --pubkey <PUBKEY> --message "Verify me" --signature <SIGNATURE> --json >/dev/null && echo "valid"

# Derive a new wallet file from an existing mnemonic
jq -r '.mnemonic' wallet.json | solana-tools-lite gen --mnemonic - --output derived.json --force
```

<details>
<summary><strong>Command reference</strong></summary>

Global flags:
- `--json` Output as JSON (pretty) [env: `SOLANA_TOOLS_LITE_JSON`]

#### `gen`
- `--mnemonic <FILE>` Read mnemonic from file or stdin (`-`)
- `--passphrase <FILE>` Read passphrase from file or stdin (`-`)
- `--unsafe-show-secret` Print secret to stdout (unsafe)
- `-o, --output <FILE>` Output wallet path
- `-f, --force` Overwrite output file [env: `SOLANA_TOOLS_LITE_FORCE`]

#### `sign`
- `-m, --message <TEXT>` Message to sign (inline)
- `--from-file <FILE>` Read message from file or stdin (`-`)
- `-k, --keypair <FILE>` Keypair path [env: `SOLANA_SIGNER_KEYPAIR`]
- `-o, --output <FILE>` Output signature path
- `-f, --force` Overwrite output file [env: `SOLANA_TOOLS_LITE_FORCE`]

#### `verify`
- `-m, --message <TEXT>` Message to verify (inline)
- `--from-file <FILE>` Read message from file or stdin (`-`) (alias: `--message-file`)
- `-s, --signature <BASE58>` Signature to verify (inline)
- `--signature-file <FILE>` Read signature from file or stdin (`-`)
- `--pubkey <BASE58>` Public key (inline)
- `--pubkey-file <FILE>` Read public key from file or stdin (`-`)
- `-o, --output <FILE>` Output report path
- `-f, --force` Overwrite output file [env: `SOLANA_TOOLS_LITE_FORCE`]

#### `base58`
- `encode -i, --input <TEXT>`
- `decode -i, --input <TEXT>`

#### `sign-tx`
- `-i, --input <FILE>` Input transaction (JSON/Base64/Base58)
- `--tables <FILE>` ALT tables file (JSON map)
- `-k, --keypair <FILE>` Keypair path [env: `SOLANA_SIGNER_KEYPAIR`]
- `-o, --output <FILE>` Output signed tx path
- `--output-format <json|base64|base58>` Force output format [env: `SOLANA_TOOLS_LITE_OUTPUT_FORMAT`]
- `-f, --force` Overwrite output file [env: `SOLANA_TOOLS_LITE_FORCE`]
- `-y, --yes` Auto-approve (no prompt) [env: `SOLANA_TOOLS_LITE_YES`]
- `--max-fee <LAMPORTS>` Fail if fee exceeds limit [env: `SOLANA_TOOLS_LITE_MAX_FEE`]
- `--summary-json` Emit signing summary JSON to stdout (requires `--output`)

#### `analyze`
- `-i, --input <FILE>` Input transaction (JSON/Base64/Base58)
- `--tables <FILE>` ALT tables file (JSON map)
- `-p, --pubkey <BASE58>` Public key to analyze as (defaults to first signer)
- `--summary-json` Emit analysis summary JSON to stdout

</details>

## ⚙️ Configuration (Environment Variables)

Solana-tools-lite supports environment variables for seamless CI/CD integration. This allows you to configure behavior globally without repeating flags.

Available variables:
- `SOLANA_SIGNER_KEYPAIR` Default keypair path
- `SOLANA_TOOLS_LITE_MAX_FEE` Default max fee (lamports)
- `SOLANA_TOOLS_LITE_OUTPUT_FORMAT` Default output format (`json|base64|base58`)
- `SOLANA_TOOLS_LITE_JSON` Enable `--json` globally (`1`/`true`)
- `SOLANA_TOOLS_LITE_FORCE` Enable `--force` globally (`1`/`true`)
- `SOLANA_TOOLS_LITE_YES` Enable `--yes` globally (`1`/`true`)

<details>
<summary><strong>How to use env defaults</strong></summary>

Environment variables act as defaults for CLI flags (you can still override with explicit flags).

Examples:

```bash
export SOLANA_SIGNER_KEYPAIR=wallet.json
solana-tools-lite sign -m "test message"
solana-tools-lite sign-tx --input unsigned.json --output signed.json --yes
```

</details>

## 🦀 Rust Library Usage

A minimal end-to-end example (parse → sign → serialize), without pulling the Solana SDK:

```rust
use solana_tools_lite::data_encoding::BASE64;
use solana_tools_lite::crypto::signing::{keypair_from_seed, SigningKey};
use solana_tools_lite::handlers::handle_sign_transaction;
use solana_tools_lite::serde::input_tx::parse_input_transaction;
use solana_tools_lite::codec::serialize_transaction;

fn sign_base64_tx(unsigned_b64: &str, signer: &SigningKey) -> anyhow::Result<String> {
    let input = parse_input_transaction(Some(unsigned_b64))?;
    let signed = handle_sign_transaction(input, signer)?;
    let raw = serialize_transaction(&signed.signed_tx);
    
    Ok(BASE64.encode(&raw))
}

fn main() -> anyhow::Result<()> {
    let signer = keypair_from_seed(&[1u8; 32])?;
    // The unsigned tx must include the signer's pubkey in account_keys.
    let unsigned = std::fs::read_to_string("unsigned_v0.b64")?;
    let signed_b64 = sign_base64_tx(&unsigned, &signer)?;
    
    println!("{signed_b64}");
    Ok(())
}
```

API docs: [docs.rs/solana-tools-lite](https://docs.rs/solana-tools-lite)

## 🧩 Extensions (Protocol Analysis)

`solana-tools-lite` supports pluggable protocol analyzers to provide enhanced insights for complex interactions:

### Light Protocol (ZK Compression)
- **Deep Analysis:** Detects compressed state operations (Confidential transfers, Compress/Decompress).
- **Privacy Classification:**
  - 🟢 **Public:** Standard transparent transaction.
  - 🟡 **Compressed:** Storage optimization only (public -> private state).
  - 🟠 **Hybrid:** Mixed operations (e.g., Transfer2/Bridge) involving both public and private state.
  - 🔴 **Confidential:** Fully private value transfers (shielded).

