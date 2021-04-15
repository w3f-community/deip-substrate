---
title: Testing Guide
---

This page will guide you through the steps needed to install, compile, run and test application.

## Setup 

First, complete the [basic Rust setup instructions](./doc/rust-setup.md).

Use Rust's native `cargo` command to build and launch the node:

```sh
cargo run --release -- --dev --tmp
```

or use `make` alias

```sh
make run
```

### Run in Docker

First, install [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/).

Then run the following command to start a single node development chain.

```bash
../scripts/docker_run.sh
```

This command will firstly compile your code, and then start a local development network. You can
also replace the default command (`cargo build --release && ./target/release/node-template --dev --ws-external`)
by appending your own. A few useful ones are as follow.

```bash
# Run Substrate node without re-compiling
../scripts/docker_run.sh ./target/release/node-template --dev --ws-external

# Purge the local dev chain
../scripts/docker_run.sh ./target/release/node-template purge-chain --dev

# Check whether the code is compilable
../scripts/docker_run.sh cargo check
```

## Automation Testing

The `make tests` command will launch comprehensive test suite. 

To launch only pallet tests run 

```sh
cargo test -p pallet-deip
```

## Manual testing

Launch node

```
make run
```

Use this [link](https://polkadot.js.org/apps/#/extrinsics?rpc=ws://127.0.0.1:9944) to open the Polkadot JS Apps UI and automatically configure the UI to connect to the local node.

Go to Setting > Developer and insert content of `../pallets/deip/src/types.json` into input and save. This is necessary in order to UI understand unknown types.

### Basic working group management (DAO)

For basic working group management managment we are integrated [multisig pallet](https://docs.rs/pallet-multisig/3.0.0/pallet_multisig/). 

To create Account go to Accounts > Accounts > + Multisig. 

All Exterenics below can be executed on behalf Multisig account as a working group.

### Project and IP management

Go to Developer > Exterenics. 

Each following step required submit signed transaction.

1. Create Domain

Save Domain hash into blockchain.

Pallet: deip 
Account: Choose any account with positive balance 
Function: `addDomain(domain)`

Payload:
```json
{
  "domain": "0x5d9118ffa9240b10fba1c217a335a26928e303b5"
}
```

2. Create Project 

Pallet: deip 
Account: Choose any account with positive balance 
Function: `createProject(project)`

Payload:
```json
{
    "external_id": "0x9238afa52e3d5013df0a150ad5250db821710981",
    "description": "0x799a5268691a263a30c17472e8e481f9a1cfc2f71c4f4ed50110be7ddde750f7",
    "domains": [
        "0x5d9118ffa9240b10fba1c217a335a26928e303b5"
    ],
    "is_private": false,
    "members": [
    "Alice"
    ],
	"team_id": "Alice"
}
```

2. Update Project 

Pallet: deip 
Account: Choose any account with positive balance 
Function: `updateProject(project_id, description, is_private, members)`

Payload:
```json
{
    "external_id": "0x9238afa52e3d5013df0a150ad5250db821710981",
    "description": "0x00f0ffa8b7badac537b2f50511768ad557a67bbb9389df3147fba0df1d0511c4",
    "is_private": true
}
```

### IP registration

3. Create Project Content aka IP asset

Pallet: deip 
Account: Choose any account with positive balance 
Function: `createProjectContent(content)`

Payload:
```json
{
    "external_id": "0xd3bb659f8afeb3697aa5ba5247cab177e5abb61b",
    "project_external_id": "0x9238afa52e3d5013df0a150ad5250db821710981",
    "content_type": "Announcement",
    "references": [],
    "description": "0xde894af8072ccbdd452ce77b9fe10c41eb0e23bbfa7b05a6567aa83822edb5d4",
    "content": "0x05c825fab16b446568f587fdd412c3baa2d6dc830edaf838c4be5623868a2110",
    "authors": [
    "Alice"
    ],
	"team_id": "Alice"
}
```

###	Access control

Manage access permissions to specific IP asset with unique Proof of Share entries that confirm a specific user was granted access to an asset.

4. Create Project NDA

Pallet: deip 
Account: Choose any account with positive balance 
Function: `createProjectNda(external_id, end_date, contract_hash, maybe_start_date, parties, projects)`

Payload:
```json
{
    "external_id": "0x267c307f87ecf9b59ed18992ea2ce9268a5c6116",
    "contract_hash": "0x228e8beff3dd666d177c502992f80ab9bf7f2a2f1f708ac540506ff6ab278dc2",
    "end_date": 1671236640000, //December 17, 2022 03:24:00
    "parties": [
        "Alice"
    ],
    "projects": [
        "0x9238afa52e3d5013df0a150ad5250db821710981"
    ],
}
```

5. Create NDA Content Access Request

Pallet: deip 
Account: Choose any account with positive balance 
Function: `createNdaContentAccessRequest(external_id, nda_external_id, encrypted_payload_hash, encrypted_payload_iv)`

Payload:
```json
{
    "external_id": "0x819a95ca97526ec5c559b5bb09f895b63a9dfef9",
	"nda_external_id": "0x267c307f87ecf9b59ed18992ea2ce9268a5c6116",
    "encrypted_payload_hash": "0xc485ed302460ac3c3f40a6e005afbb43b63147295c0f7526450a2948269b2ba1",
    "encrypted_payload_iv": "aae1889ccece4e147b8186f82ef4ff5e89056bbc"
}
```

6. Fulfill or reject NDA Content Access Request

Pallet: deip 
Account: Choose any account with positive balance 
Function: `createNdaContentAccessRequest(external_id, nda_external_id, encrypted_payload_hash, encrypted_payload_iv)`

Payload:
```json
{
    "external_id": "0x819a95ca97526ec5c559b5bb09f895b63a9dfef9",
    "encrypted_payload_encryption_key": "DEIP5jQLPR8unEnycui7H2kPPBqwwVJoVuVpsxAvvBU3R2zjVo3dPa",
    "proof_of_encrypted_payload_encryption_key": "a784f681975df32559381d21f67c604b99717e80"
}
```

OR

Pallet: deip 
Account: Choose any account with positive balance 
Function: `rejectNdaContentAccessRequest(external_id)`

Payload:
```json
{
    "external_id": "0x819a95ca97526ec5c559b5bb09f895b63a9dfef9"
}
```