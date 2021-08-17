use crate::actor::*;
use super::actor_io::*;

use substrate_subxt::{Client, ChainBlock};
use substrate_subxt::system::System;
use jsonrpsee_ws_client::Subscription;

use crate::RuntimeT;

pub struct BlockchainActor {
    client: Option<Client<RuntimeT>>,
}
impl BlockchainActor {
    pub fn new() -> Self {
        Self { client: None }
    }
}

pub enum BlockchainActorInputData {
    SubscribeFinalizedBlocks,
    GetBlockHash(<RuntimeT as System>::BlockNumber),
    GetBlock(<RuntimeT as System>::Hash),
    SetClient(Client<RuntimeT>),
    GetBlockEvents(<RuntimeT as System>::Hash),
}
impl BlockchainActorInputData {
    pub fn subscribe_finalized_blocks() -> BlockchainActorInput {
        ActorDirective::Input(Self::SubscribeFinalizedBlocks)
    }
    pub fn get_block_hash(number: <RuntimeT as System>::Header) -> BlockchainActorInput {
        ActorDirective::Input(Self::GetBlockHash(number.number))
    }
    pub fn get_block(hash: <RuntimeT as System>::Hash) -> BlockchainActorInput {
        ActorDirective::Input(Self::GetBlock(hash))
    }
    pub fn set_client(client: Client<RuntimeT>) -> BlockchainActorInput {
        ActorDirective::Input(Self::SetClient(client))
    }
    pub fn get_block_events(hash: <RuntimeT as System>::Hash) -> BlockchainActorInput {
        ActorDirective::Input(Self::GetBlockEvents(hash))
    }
}
pub type BlockchainActorInput = ActorDirective<BlockchainActorInputData>;
pub type FinalizedBlocksSubscription = Subscription<<RuntimeT as System>::Header>;
pub enum BlockchainActorOutput {
    NoClient,
    Ok(BlockchainActorOutputData)
}
pub enum BlockchainActorOutputData {
    SubscribeFinalizedBlocks(Result<FinalizedBlocksSubscription, substrate_subxt::Error>),
    GetBlockHash(Result<Option<<RuntimeT as System>::Hash>, substrate_subxt::Error>),
    GetBlock(Result<Option<ChainBlock<RuntimeT>>, substrate_subxt::Error>),
    SetClient,
    GetBlockEvents(Result<Vec<RawEvent>, substrate_subxt::Error>),
}
pub type BlockchainActorIO = ActorJack<BlockchainActorInput, BlockchainActorOutput>;
pub type BlockchainActorIOPair = ActorJackPair<BlockchainActorIO, BlockchainActorInput, BlockchainActorOutput>;

#[async_trait::async_trait]
impl Actor
<
    BlockchainActorInputData,
    BlockchainActorInput,
    BlockchainActorOutput,
    BlockchainActorIO
>
for BlockchainActor
{
    async fn on_input(&mut self, data: BlockchainActorInputData) -> BlockchainActorOutput {
        if self.client.is_none() {
            return if let BlockchainActorInputData::SetClient(c) = data {
                let _ = self.client.replace(c);
                BlockchainActorOutput::Ok(BlockchainActorOutputData::SetClient)
            } else { 
                BlockchainActorOutput::NoClient
            };
        }
        let client = self.client.as_mut().unwrap();
        let output = match data {
            BlockchainActorInputData::SubscribeFinalizedBlocks => {
                BlockchainActorOutputData::SubscribeFinalizedBlocks(
                    client.subscribe_finalized_blocks().await)
            },
            BlockchainActorInputData::GetBlockHash(number) => {
                BlockchainActorOutputData::GetBlockHash(
                    client.block_hash(Some(number.into())).await)
            },
            BlockchainActorInputData::GetBlock(hash) => {
                BlockchainActorOutputData::GetBlock(
                    client.block(Some(hash)).await)
            },
            BlockchainActorInputData::SetClient(c) => {
                let _ = std::mem::replace(client, c);
                BlockchainActorOutputData::SetClient
            },
            BlockchainActorInputData::GetBlockEvents(hash) => {
                let events = get_block_events(client, client.events_decoder(), hash).await;
                BlockchainActorOutputData::GetBlockEvents(events)
            },
        };
        BlockchainActorOutput::Ok(output)
    }
}

use sp_core::storage::StorageKey;
use sp_core::hashing::twox_128;

use substrate_subxt::{RawEvent, Phase, RuntimeError, Raw};
use substrate_subxt::{EventsDecoder, Rpc};

struct SystemEvents(StorageKey);
impl SystemEvents {
    pub(crate) fn new() -> Self {
        let mut storage_key = twox_128(b"System").to_vec();
        storage_key.extend(twox_128(b"Events").to_vec());
        log::debug!("Events storage key {:?}", hex::encode(&storage_key));
        Self(StorageKey(storage_key))
    }
}

impl From<SystemEvents> for StorageKey {
    fn from(key: SystemEvents) -> Self {
        key.0
    }
}

async fn get_block_events(
    client: &Client<RuntimeT>,
    decoder: &EventsDecoder<RuntimeT>,
    // header: &<RuntimeT as System>::Header,
    hash: <RuntimeT as System>::Hash
)
    -> Result<Vec<RawEvent>, substrate_subxt::Error>
{
    let change_set = client
        // .query_storage_at(&[SystemEvents::new().into()], Some(header.hash()))
        .query_storage_at(&[SystemEvents::new().into()], Some(hash))
        .await?;
    
    let mut events = Vec::new();
    
    for (_key, data) in change_set.into_iter().map(|x| x.changes).flatten() {
        if let Some(data) = data {
            let raw_events = match decoder.decode_events(&mut &data.0[..]) {
                Ok(events) => events,
                Err(error) => return Err(error),
            };
            for (phase, raw) in raw_events {
                if let Phase::ApplyExtrinsic(i) = phase {
                    let event = match raw {
                        Raw::Event(event) => event,
                        Raw::Error(err) => return Err(err.into()),
                    };
                    events.push(event);
                }
            }
        }
    }
    Ok(events)
}
