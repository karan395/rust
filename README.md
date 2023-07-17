<p align="center">
  <img src="/peerlogo.jpg">
</p><h1><code>PEER</code></h1>

 <strong> PEER is a next-generation blockchain innovation.</strong> 🚀.



## Contributions & Code of Conduct

Please follow the contributions guidelines as outlined in [`docs/CONTRIBUTING.adoc`](docs/CONTRIBUTING.adoc). In all communications and contributions, this project follows the [Contributor Covenant Code of Conduct](docs/CODE_OF_CONDUCT.md).

## Security

The security policy and procedures can be found in [`docs/SECURITY.md`](docs/SECURITY.md).

## License

- PEER Primitives (`sp-*`), Frame (`frame-*`) and the pallets (`pallets-*`), binaries (`/bin`) and all other utilities are licensed under [Apache 2.0](LICENSE-APACHE2).
- PEER Client (`/client/*` / `sc-*`) is licensed under [GPL v3.0 with a classpath linking exception](LICENSE-GPL3).

The reason for the split-licensing is to ensure that for the vast majority of teams using 5irechain to create feature-chains, then all changes can be made entirely in Apache2-licensed code, allowing teams full freedom over what and how they release and giving licensing clarity to commercial teams.

In the interests of the community, we require any deeper improvements made to 5irechain's core logic (e.g. 5irechain's internal consensus, crypto or database code) to be contributed back so everyone can benefit.

## Features

This template includes the minimum required components to start a NPoS testnet.

* Consensus related pallets: Babe & GRANDPA
* Staking related pallets: staking, session, authorship, im-online, offences, utility
* Governance related pallets: collective, membership, elections-phragmen, democracy, treasure
* xcm 


## Getting Started

Follow the steps below to get started.

### Rust Setup

First, complete the Dev Docs Installation.

### Build and Run

Use the following command to build the node and run it after build successfully:

sh
cargo build --release
./target/release/peer --dev --tmp
