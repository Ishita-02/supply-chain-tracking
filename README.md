# Supply Chain Tracking Dapp

StellarTrack is a decentralized application (dApp) developed on the Stellar blockchain, dedicated to transforming supply chain tracking. Utilizing blockchain technology, StellarTrack ensures transparency, immutability, and security throughout the supply chain process.

StellarTrack enables users to track the movement of products from their origin to their final destination. Every transaction and transfer of ownership is securely recorded on the Stellar blockchain, guaranteeing authenticity and reliability. This transparent ledger facilitates trust among supply chain partners and facilitates swift resolution of any issues.

StellarTrack provides an intuitive interface for stakeholders to access essential data and insights about their goods. Whether it's tracing the origins of raw materials or monitoring the progress of finished products, StellarTrack offers comprehensive visibility into the supply chain.

By leveraging the capabilities of the Stellar network, StellarTrack aims to streamline supply chain operations, mitigate risks, and enhance efficiency in global trade.

# Soroban Project

## Project Structure

This repository uses the recommended structure for a Soroban project:
```text
.
├── contracts
│   └── supply_chain
│       ├── src
│       │   ├── lib.rs
│       │   └── test.rs
│       └── Cargo.toml
├── Cargo.toml
└── README.md
```

- New Soroban contracts can be put in `contracts`, each in their own directory. There is already a `hello_world` contract in there to get you started.
- If you initialized this project with any other example contracts via `--with-example`, those contracts will be in the `contracts` directory as well.
- Contracts should have their own `Cargo.toml` files that rely on the top-level `Cargo.toml` workspace for their dependencies.
- Frontend libraries can be added to the top-level directory as well. If you initialized this project with a frontend template via `--frontend-template` you will have those files already included.
