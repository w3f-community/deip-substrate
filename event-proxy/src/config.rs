use std::path::Path;
use std::io;
use std::fs;
use serde::{Deserialize, de::DeserializeOwned};

pub fn load<Offchain: DeserializeOwned, P: AsRef<Path>>(path: P) -> io::Result<Config<Offchain>> {
    toml::from_slice(fs::read(path)?.as_slice()).map_err(Into::into)
} 

#[derive(Deserialize)]
pub struct Config<Offchain> {
    pub blockchain: Blockchain,
    pub kafka: Kafka,
    pub offchain: Offchain
}

#[derive(Deserialize)]
pub struct Blockchain {
    pub rpc: String
}

#[derive(Deserialize)]
pub struct Kafka {
    pub bootstrap_servers: String,
}

#[derive(Deserialize)]
pub struct Offchain<LastKnownBlock> {
    pub last_known_block: LastKnownBlock
}
