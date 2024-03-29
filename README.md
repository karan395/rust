<h1></p>PEER</code></h1>

PEER is a next-generation blockchain innovation. It is a L0 chain that allows the addition of parachains to the blockchain. It is also compatible with Ethereum as it has EVM (Ethereum Virtual Machine) integrated.

<p align="center">
  <img src="/peer.jpg">
</p>

# Table of Contents
* Description
* Contributions & Code of Conduct
* Features
* Getting Started
* Rust Setup
* Build and Run


# Description
PEER is a next-generation blockchain innovation that enables the addition of parachains to the blockchain. It offers compatibility with Ethereum through the integration of EVM. This blockchain solution provides advanced features and aims to revolutionize the decentralized ecosystem.


# Features
PEER incorporates the following features:

- Consensus related pallets: Babe & GRANDPA.
- Staking related pallets: staking, session, authorship, im-online, offences, utility.
- Governance related pallets: collective, membership, elections-phragmen, democracy, treasure.
- xcm.
  These components form the minimum required setup to start a NPoS (Nominated Proof-of-Stake) testnet.

# Getting Started
Follow the steps below to get started with PEER.

### Rust Setup
Before building and running PEER, ensure you have completed the [Dev Docs](https://docs.substrate.io/install/) Installation. This will set up the necessary Rust environment.

### Build and Run
To build the node and run it after a successful build, use the following command:

```sh
Copy code
cargo build --release
./target/release/peer --dev --tmp
