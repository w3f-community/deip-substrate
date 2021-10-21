
# DEIP Network

DEIP Network is a Substrate-based implementation of Creator Economy Protocol. It is built as an application-specific blockchain that can provide end-to-end management of IP assets as NFTs and become a base asset for the entire DeFi stack. DEIP Network aims to become the major Web3 protocol for intellectual capital management and therefore aggregate various IP assets in the network.

https://www.youtube.com/watch?v=hmfaRN76XTY

# DEIP Constructor

Main aim of DEIP Constructor is to drive adoption of DEIP Network and help gather tokenised IP assets in the network. DEIP Constructor is a modular open-source framework, that allows to build IP-centric Web3 platform in weeks. DEIP Constructor has two modes to operate  1) no-code mode and 2) low-code mode.

# No-Code mode

Use admin panel and setup wizard. Customised UI with drag-and-drop functionality. Integrate existing modules like F-NFT, DeFi, Funding, Licensing, DAO, etc. In future third party modules will be provided by other Polkadot projects, and will be able to install as a Pallet.

# Low-code mode

Use WASM and Rust to build custom modules. Customised existing modules by digging deeper into the code. In future we will also provide our own DSL and Workflow builder. Open API for integration with third-party services.


## Getting Started

Follow these steps to get started :hammer_and_wrench:

### Rust Setup

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

### Run

Use Rust's native `cargo` command to build and launch the node:

```sh
cargo run --release -- --dev --tmp
```

or use `make` alias

```sh
make run
```

### Build

The `cargo run` command will perform an initial build. Use the following command to build the node
without launching it:

```sh
cargo build --release
```


### Test 

The `make tests` command will launch comprehensive test suite. 

### Embedded Docs

Once the project has been built, the following command can be used to explore all parameters and
subcommands:

```sh
./target/release/node-template -h
```

To build and open rust doc: 

```sh
cargo doc --package <spec> --open 
```

Replacing <spec> with one of the included pallets (i.e. cargo doc --package pallet-deip --open).

## Run

The provided `cargo run` command will launch a temporary node and its state will be discarded after
you terminate the process. After the project has been built, there are other ways to launch the
node.
## Connect UI

There are 2 options: 

1. Use [Substrate Front End Template](https://github.com/substrate-developer-hub/substrate-front-end-template). Follow instrations in the repo.
2. Use this [link](https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944) to open the Polkadot JS Apps UI and automatically configure the UI to connect to the local node.

## Tools 

1. [Utilities and libraries](https://polkadot.js.org/docs/) for interacting with the Polkadot/Parachains/Substrate network from JavaScript
2. [Substrate Utilities](https://www.shawntabrizi.com/substrate-js-utilities/)
   

### Single-Node Development Chain

This command will start the single-node development chain with persistent state:

```bash
./target/release/node-template --dev
```

Purge the development chain's state:

```bash
./target/release/node-template purge-chain --dev
```

Start the development chain with detailed logging:

```bash
RUST_LOG=debug RUST_BACKTRACE=1 ./target/release/node-template -lruntime=debug --dev
```

### Multi-Node Local Testnet

If you want to see the multi-node consensus algorithm in action, refer to
[our Start a Private Network tutorial](https://substrate.dev/docs/en/tutorials/start-a-private-network/).
    
        
## Operations 
An operation is the smallest unit of the protocol dedicated to modify the state of a blockchain relying on predefined rules. Multiple operations can be combined into a transaction to be executed atomically. A composition of operations inside a transaction can reflect a specific business process. This allows Portals to set up and build services to achieve their special goals.

Every operation is designed with a built-in authorization layer, which defines what accounts are permitted to execute it. An account can belong either to a person or an organization (DAO) with multi-signature mode, as well as other governance models.
The list below describes supported operations and will receive ongoing updates as the protocol evolves.
    
### Account operations
    
  * ``` CREATE_ACCOUNT ``` - An operation that allows to create a new account. It is used to register a new user or DAO in the network with a set of keys, options, and metadata. This operation requires a fee from the creator. 
  * ``` UPDATE_ACCOUNT ``` - An operation that allows to update existing account options, metadata, and keys set. 
    
### Project operations
    
  * ``` CREATE_PROJECT ``` - An operation that allows to create a new project. The project is the central entity in the protocol which aggregates intangible asset content. It provides opportunities to discover, manage, and work on intangible assets. Projects can be owned and governed by assigned teams that may include DAOs and individual accounts.  
  * ``` UPDATE_PROJECT ``` - An operation that allows to update existing project options and metadata.
  * ``` ADD_PROJECT_CONTENT ``` - An operation that allows to add new segregated content to an intangible asset. Evaluation of an intangible asset depends on how valuable its content is. Measurement of this value is one of the main challenges of the DEIP assessment system (DAS).  
  * ``` START_PROJECT_FUNDRAISING ``` - An operation that allows intangible asset owners to start a fundraising campaign and attract investment for further activities in exchange for project NFT/F-NFT assets. Each fundraising campaign may use one of the supported models that describes applicable rules such as who is permitted to contribute the fundraising, tokens distribution model, soft/hard cap, refund policy, and more. 
  * ``` CONTRIBUTE_PROJECT_FUNDRAISING ``` - An operation that allows to contribute to project fundraising campaigns and make investments. Depending on the fundraising campaign model, the execution of this operation may be narrowed down to a limited number of accounts. 
  * ``` CREATE_PROJECT_LICENSE ``` - An operation that allows to create a license for an intangible asset to give the licensee the right to use it for commercial or other purposes.
  * ``` CREATE_PROJECT_NDA ``` - An operation that allows to create a non-disclosure agreement (NDA) for an intangible asset between involved parties. This operation gives the opportunity to establish a channel for sharing confidential information about an intangible asset and its content according to specified conditions.  
  * ``` CREATE_PROJECT_CONTENT_REVIEW ``` - An operation that allows to make an assessment of an intangible asset and helps to define its value. The assessment model is customizable and represents a set of criteria that a reviewer addresses during their assessment. This operation is an essential unit of the DEIP assessment system (DAS).
  * ``` SUPPORT_PROJECT_CONTENT_REVIEW ``` - An operation that allows to support an existing assessment of an intangible asset and hence increase or decrease its value index. This operation is used by curators that help to justify the correctness of the assessment. As in the previous operation, this is an important part of the DEIP assessment system (DAS).
    
### Asset operations
    
  * ``` CREATE_ASSET ``` - An operation that allows to create an asset of a specific type. The type of asset must be specified while its creation and can not be changed. This operation gives opportunities to: tokenize an intangible asset or DAO by issuing non-fungible tokens (NFT/F-NFT); register an asset-backed token or dX stablecoin; or create a custom token to facilitate business models for custom services. Every asset has a set of options that may enable or disable specific actions for the asset such as transfer restrictions and others. 
  * ``` ISSUE_ASSET ``` - An operation that allows to issue previously created assets in a specific account. The amount of assets that can be issued is limited by the MAX amount value specified during asset creation and may be limited by additional options. 
  * ``` RESERVE_ASSET ``` - An operation that allows to burn previously issued assets according to the rules specified during asset creation. This operation may be restricted depending on asset options.
  * ``` TRANSFER ``` - An operation that allows to transfer assets between accounts. Depending on the asset, the operation may require a fee from the sender.
    
### Proposal operations
    
  * ``` CREATE_PROPOSAL ``` - An operation that allows to create a postponed on-chain transaction. As mentioned previously, a transaction can be composed of an arbitrary number of operations that may require approval from multiple accounts. To address complex workflows, where approval of a transaction may last for a long time while all required signatures are being collected, DEIP protocol provides the ability to keep such a transaction in a pending state until its execution.
  * ``` UPDATE_PROPOSAL ``` - An operation that allows to add or revoke approval of a specific account for a specified proposal. Once all required approvals are collected, the proposed transaction is executed.
  * ``` DELETE_PROPOSAL ``` - An operation that allows to reject a proposed transaction and delete it from the pending transactions pool. This operation may only be executed by an account whose approval is required for the proposed transaction.


## Template Structure

A Substrate project such as this consists of a number of components that are spread across a few
directories.

### Node

A blockchain node is an application that allows users to participate in a blockchain network.
Substrate-based blockchain nodes expose a number of capabilities:

-   Networking: Substrate nodes use the [`libp2p`](https://libp2p.io/) networking stack to allow the
    nodes in the network to communicate with one another.
-   Consensus: Blockchains must have a way to come to
    [consensus](https://substrate.dev/docs/en/knowledgebase/advanced/consensus) on the state of the
    network. Substrate makes it possible to supply custom consensus engines and also ships with
    several consensus mechanisms that have been built on top of
    [Web3 Foundation research](https://research.web3.foundation/en/latest/polkadot/NPoS/index.html).
-   RPC Server: A remote procedure call (RPC) server is used to interact with Substrate nodes.

There are several files in the `node` directory - take special note of the following:

-   [`chain_spec.rs`](./node/src/chain_spec.rs): A
    [chain specification](https://substrate.dev/docs/en/knowledgebase/integrate/chain-spec) is a
    source code file that defines a Substrate chain's initial (genesis) state. Chain specifications
    are useful for development and testing, and critical when architecting the launch of a
    production chain. Take note of the `development_config` and `testnet_genesis` functions, which
    are used to define the genesis state for the local development chain configuration. These
    functions identify some
    [well-known accounts](https://substrate.dev/docs/en/knowledgebase/integrate/subkey#well-known-keys)
    and use them to configure the blockchain's initial state.
-   [`service.rs`](./node/src/service.rs): This file defines the node implementation. Take note of
    the libraries that this file imports and the names of the functions it invokes. In particular,
    there are references to consensus-related topics, such as the
    [longest chain rule](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#longest-chain-rule),
    the [Aura](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#aura) block authoring
    mechanism and the
    [GRANDPA](https://substrate.dev/docs/en/knowledgebase/advanced/consensus#grandpa) finality
    gadget.

After the node has been [built](#build), refer to the embedded documentation to learn more about the
capabilities and configuration parameters that it exposes:

```shell
./target/release/node-template --help
```

### Runtime

In Substrate, the terms
"[runtime](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#runtime)" and
"[state transition function](https://substrate.dev/docs/en/knowledgebase/getting-started/glossary#stf-state-transition-function)"
are analogous - they refer to the core logic of the blockchain that is responsible for validating
blocks and executing the state changes they define. The Substrate project in this repository uses
the [FRAME](https://substrate.dev/docs/en/knowledgebase/runtime/frame) framework to construct a
blockchain runtime. FRAME allows runtime developers to declare domain-specific logic in modules
called "pallets". At the heart of FRAME is a helpful
[macro language](https://substrate.dev/docs/en/knowledgebase/runtime/macros) that makes it easy to
create pallets and flexibly compose them to create blockchains that can address
[a variety of needs](https://www.substrate.io/substrate-users/).

Review the [FRAME runtime implementation](./runtime/src/lib.rs) included in this template and note
the following:

-   This file configures several pallets to include in the runtime. Each pallet configuration is
    defined by a code block that begins with `impl $PALLET_NAME::Config for Runtime`.
-   The pallets are composed into a single runtime by way of the
    [`construct_runtime!`](https://crates.parity.io/frame_support/macro.construct_runtime.html)
    macro, which is part of the core
    [FRAME Support](https://substrate.dev/docs/en/knowledgebase/runtime/frame#support-library)
    library.

### Pallets

The runtime in this project is constructed using many FRAME pallets that ship with the
[core Substrate repository](https://github.com/paritytech/substrate/tree/master/frame) and a
template pallet that is [defined in the `pallets`](./pallets/template/src/lib.rs) directory.

A FRAME pallet is compromised of a number of blockchain primitives:

-   Storage: FRAME defines a rich set of powerful
    [storage abstractions](https://substrate.dev/docs/en/knowledgebase/runtime/storage) that makes
    it easy to use Substrate's efficient key-value database to manage the evolving state of a
    blockchain.
-   Dispatchables: FRAME pallets define special types of functions that can be invoked (dispatched)
    from outside of the runtime in order to update its state.
-   Events: Substrate uses [events](https://substrate.dev/docs/en/knowledgebase/runtime/events) to
    notify users of important changes in the runtime.
-   Errors: When a dispatchable fails, it returns an error.
-   Config: The `Config` configuration interface is used to define the types and parameters upon
    which a FRAME pallet depends.

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
./scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
./scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
./scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
./scripts/docker_run.sh cargo check
```
