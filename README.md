# Sui Package Verifier CLI

A command-line tool to verify Sui Move package source against on-chain bytecode. 
This tool serves as a CLI wrapper around MystenLabs' [`sui-source-validation`](https://github.com/MystenLabs/sui/tree/main/crates/sui-source-validation) package, providing a convenient interface for source code verification.

## Prerequisites

Before using this tool, ensure your Sui Move package meets these requirements:

### 1. Package Address Configuration
Your `Move.toml` must contain the published package address in the `[addresses]` section:
```toml
[addresses]
deepbook_wrapper = "0x85f2cfacbd51ba2aa73397531cc7aea73a2ee4cda574926f809d62e088533805"
```

### 2. Published-at Field
The package must include a `published-at` field in the `[package]` section, matching your published package address:
```toml
[package]
name = "deepbook_wrapper"
version = "0.0.1"
edition = "2024.beta"
published-at = "0x85f2cfacbd51ba2aa73397531cc7aea73a2ee4cda574926f809d62e088533805"
```

### 3. Dependencies Configuration
Each dependency must have:
- A specified address in the `[addresses]` section
- A `published-at` field in its own `Move.toml`

Example dependency's `Move.toml`:
```toml
[package]
name = "deepbook"
edition = "2024.beta"
version = "0.0.1"
published-at = "0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809"

[addresses]
deepbook = "0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809"
```

### 4. Local Dependencies Setup
**Important**: Currently, the tool only works with locally specified dependencies. All dependencies must be available locally and properly configured with their addresses in your package's `Move.toml`:
```toml
[dependencies]
Sui = { git = "https://github.com/MystenLabs/sui.git", subdir = "crates/sui-framework/packages/sui-framework", rev = "framework/mainnet" }
deepbook = { local = "../deepbook", address = "0x2c8d603bc51326b8c13cef9dd07031a408a48dddb541963357661df5d3204809" }
token = { local = "../token", address = "0xdeeb7a4662eec9f2f3def03fb937a663dddaa2e215b8078a284d026b7946c270" }
```

## Basic Usage

### Verify a package
```bash
sui-package-verify --package-path /path/to/your/package
```
### Verify against testnet
```bash
sui-package-verify --package-path /path/to/your/package --network testnet
```
### Use a custom RPC URL
```bash
sui-package-verify --package-path /path/to/your/package --rpc-url https://your-custom-rpc.example.com
```
### Enable verbose logging

```bash
sui-package-verify --package-path /path/to/your/package --verbose
```

## Options

| Option | Description | Default |
|--------|-------------|---------|
| `--package-path PATH` | Path to the package to verify | _(required)_ |
| `--network NETWORK` | Network to verify against (mainnet, testnet, devnet, localnet) | `mainnet` |
| `--rpc-url URL` | Custom RPC URL to use instead of network presets | _(network dependent)_ |
| `--address ADDR` | Address of the package on-chain (if different from manifest) | _(from manifest)_ |
| `--verbose` | Enable verbose logging | `false` |
| `--help` | Print help information | - |
| `--version` | Print version information | - |


## Development

### Run in Development Mode
To run the package verifier in development mode with a local package:
```bash
cargo run -- --package-path /path/to/your/package
```

### Building for Production
To build an optimized release version:
```bash
cargo build --release
```
The compiled binary will be available at `target/release/sui-package-verify`

You can install it globally on your system by copying it to a directory in your PATH:
```bash
# On Unix-like systems (Linux/macOS)
sudo cp target/release/sui-package-verify /usr/local/bin/

# Or alternatively, without sudo, to your user's bin directory
cp target/release/sui-package-verify ~/.local/bin/
```

## Current Limitations

- Dependencies verification is not supported yet. The tool can only verify the main package.
- Only local dependencies are supported in the package's `Move.toml` configuration.
- The `published-at` field requirement comes from the underlying `sui-source-validation` package which we use for verification.
- Some limitations are inherited from the `sui-source-validation` package as we're using it under the hood.