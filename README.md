# Bitcoin in Rust

A simplified Bitcoin implementation in Rust

## Overview

This Cargo workspace contains four components: a core Bitcoin library (`btclib`), and implementations for a miner, node, and wallet.

The core library implements basic blockchain functionality including ECDSA cryptography with secp256k1, SHA-256 hashing, UTXO tracking, transaction validation, proof-of-work mining, and a mempool with fee prioritization. Blocks and transactions use CBOR serialization for persistence.

## Usage

### First-Time Setup

Before running the system, you need to generate cryptographic keys and a wallet configuration file.

1.  **Generate Keys and Config:**
    Run the following commands in your terminal to create two key pairs (`alice` and `bob`) and a default `wallet_config.toml`.

    ```sh
    # Generate keys
    cargo run --bin key_gen alice
    cargo run --bin key_gen bob

    # Generate default wallet config
    cargo run --bin wallet -- generate-config
    ```

2.  **Edit `wallet_config.toml`:**
    Open the generated `wallet_config.toml` and add one of the key pairs to the `my_keys` section to assign ownership to your wallet.

    ```toml
    # Tell the wallet that the "alice" keys belong to us
    my_keys = [
      { public = "alice.pub.pem", private = "alice.priv.cbor" },
    ]
    ```

### Running the System

Run each component in a **separate terminal window** from the project's root directory.

1.  **Start the Node:**

    ```sh
    cargo run --bin node
    ```

2.  **Start the Miner:**
    Connect the miner to the node and assign a public key to receive block rewards.

    ```sh
    cargo run --bin miner -- --address 127.0.0.1:9000 --public-key-file alice.pub.pem
    ```

3.  **Start the Wallet:**
    The wallet will load your config and connect to the node.

    ```sh
    cargo run --bin wallet
    ```

    The TUI will appear and your balance will update as the miner finds blocks. Press `Esc` to access the menu to send funds or quit.
