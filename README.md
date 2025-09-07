# Bitcoin in Rust

A simplified Bitcoin implementation in Rust

## Overview

This Cargo workspace contains four components: a core Bitcoin library (`btclib`), and stub implementations for miner, node, and wallet functionality.

The core library implements basic blockchain functionality including ECDSA cryptography with secp256k1, SHA-256 hashing, UTXO tracking, transaction validation, proof-of-work mining, and a mempool with fee prioritization. Blocks and transactions use CBOR serialization for persistence.

## Usage

```bash
cargo build    # Build workspace
cargo test     # Run tests
```

## Progress

**Completed (btclib)**
- [x] Blockchain structure with blocks and headers
- [x] Transaction system with inputs/outputs
- [x] ECDSA cryptography (secp256k1)
- [x] SHA-256 hashing and Merkle trees
- [x] UTXO tracking and validation
- [x] Proof-of-work mining algorithm
- [x] Mempool with fee prioritization
- [x] Difficulty adjustment mechanism
- [x] CBOR serialization/persistence

**WIP**
- [ ] Mining loop implementation
- [ ] P2P networking protocol
- [ ] Wallet key management
- [ ] CLI interfaces

**Future**
- [ ] Network consensus
- [ ] wallet 
- [ ] Performance optimizations

**Reference**

Building Bitcoin in Rust: Learn Rust Programming and the Fundamentals of Bitcoin - Lukas Hozda
