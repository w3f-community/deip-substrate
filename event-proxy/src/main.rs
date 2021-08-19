mod frame;
mod events;
mod types;
mod runtime;
mod call_serializer;
mod actor;
mod app;

use std::time::Duration;
use std::collections::VecDeque;

use substrate_subxt::{system::System};
use substrate_subxt::NodeTemplateRuntime;

use tokio::sync::mpsc;
use futures::stream::{FuturesOrdered, StreamExt};
use futures::{Future, SinkExt};

const URL: &str = "ws://localhost:9944/";

type RuntimeT = NodeTemplateRuntime;

use app::{
    Actor, ActorI, ActorO, ActorIO,
    RpcClientBuilderActor, RpcClientBuilderActorIO, RpcClientBuilderActorInput,
    MessageBrokerActor, MessageBrokerActorIO, MessageBrokerActorInput,
    BlockchainActor, BlockchainActorIO, BlockchainActorInput, BlockchainActorOutput, FinalizedBlocksSubscription, BlockchainActorOutputData, BlocksReplay, BlockEvents,
    OffchainActor, OffchainActorIO, OffchainActorInput, OffchainActorOutput, OffchainActorOutputData,
};


fn last_known_block() -> events::BlockMetadata<RuntimeT> {
    let number = 69354;
    let hash = "6389774b0ef2e24545d52802ff8d45c0a7aa7172c74b4a529119d9f79cfee7cb";
    let parent_hash = "222a4d01f62a79d395416e7a1b5f19eacf6a5edfb443e60926aa982cc9fcd424";
    events::BlockMetadata {
        number,
        hash: sp_core::H256::from_slice(hex::decode(hash).unwrap().as_slice()),
        parent_hash: sp_core::H256::from_slice(hex::decode(parent_hash).unwrap().as_slice()),
    }
}

macro_rules! reset {
    ($actor_task_queue:ident, $_released_actor_queue:ident) => {
        $actor_task_queue.push(init_actor_task::<_, _, OffchainActorIO>(
            OffchainActorInput::build_client(Ok(last_known_block())),
            &mut $_released_actor_queue
        ).await);
    };
}

macro_rules! reset_blockchain_actor {
    ($actor_task_queue:ident, $_released_actor_queue:ident) => {
        $actor_task_queue.push(init_actor_task::<_, _, RpcClientBuilderActorIO>(
            RpcClientBuilderActorInput::Input(()),
            &mut $_released_actor_queue
        ).await);
    };
}

#[tokio::main]
async fn main() {
    
    // flexi_logger::Logger::try_with_env().unwrap().start().unwrap();
    
    // Init rpc-client-builder-actor:
    let mut rpc_client_builder_actor = RpcClientBuilderActor;
    let (rpc_client_builder_actor_io, rpc_client_builder_actor_io2) = RpcClientBuilderActorIO::pair();
    tokio::spawn(async move { rpc_client_builder_actor.actor_loop(rpc_client_builder_actor_io).await });
    
    // Init blockchain-actor:
    let mut blockchain_actor = BlockchainActor::new();
    let (blockchain_actor_io, blockchain_actor_io2) = BlockchainActorIO::pair();
    tokio::spawn(async move { blockchain_actor.actor_loop(blockchain_actor_io).await });
    
    // Init message-broker-actor:
    let mut message_broker_actor = MessageBrokerActor::new();
    let (message_broker_actor_io, message_broker_actor_io2) = MessageBrokerActorIO::pair();
    tokio::spawn(async move { message_broker_actor.actor_loop(message_broker_actor_io).await });
    
    // Init offchain-actor:
    let mut offchain_actor = OffchainActor::new();
    let (offchain_actor_io, offchain_actor_io2) = OffchainActorIO::pair();
    tokio::spawn(async move { offchain_actor.actor_loop(offchain_actor_io).await });
    let mut subscription_task_queue = FuturesOrdered::new();
    let mut buffered_subscription_task_queue = FuturesOrdered::new();
    let mut blocks_replay_task_queue = FuturesOrdered::new();
    
    let mut blockchain_actor_task_queue = FuturesOrdered::new();
    let mut message_broker_actor_task_queue = FuturesOrdered::new();
    let mut rpc_client_builder_actor_task_queue = FuturesOrdered::new();
    let mut offchain_actor_task_queue = FuturesOrdered::new();
    
    let mut released_blockchain_actor_queue = released_actor_queue::<_, _, BlockchainActorIO>();
    let mut released_message_broker_actor_queue = released_actor_queue::<_, _, MessageBrokerActorIO>();
    let mut released_rpc_client_builder_actor_queue = released_actor_queue::<_, _, RpcClientBuilderActorIO>();
    let mut released_offchain_actor_queue = released_actor_queue::<_, _, OffchainActorIO>();

    release_actor(blockchain_actor_io2, &mut released_blockchain_actor_queue).await;
    release_actor(message_broker_actor_io2, &mut released_message_broker_actor_queue).await;
    release_actor(rpc_client_builder_actor_io2, &mut released_rpc_client_builder_actor_queue).await;
    release_actor(offchain_actor_io2, &mut released_offchain_actor_queue).await;
    
    // Put the initial task to trigger main workflow:
    reset!(offchain_actor_task_queue, released_offchain_actor_queue);
    
    loop { tokio::select! {
        Some(offchain_actor_task_result) = offchain_actor_task_queue.next() => {
            let (maybe_output, io) = offchain_actor_task_result;
            release_actor(io, &mut released_offchain_actor_queue).await;
            let output = if maybe_output.is_none() { unreachable!(); } else { maybe_output.unwrap() };
            match output {
                OffchainActorOutput::NoClient => {
                    offchain_actor_task_queue.push(init_actor_task::<_, _, OffchainActorIO>(
                        OffchainActorInput::build_client(Ok(last_known_block())),
                        &mut released_offchain_actor_queue
                    ).await);
                },
                OffchainActorOutput::Output(OffchainActorOutputData::BuildClient(client)) => {
                    offchain_actor_task_queue.push(init_actor_task::<_, _, OffchainActorIO>(
                        OffchainActorInput::set_client(client),
                        &mut released_offchain_actor_queue
                    ).await);
                },
                OffchainActorOutput::Output(OffchainActorOutputData::SetClient) => {
                    offchain_actor_task_queue.push(init_actor_task::<_, _, OffchainActorIO>(
                        OffchainActorInput::get_last_known_block(),
                        &mut released_offchain_actor_queue
                    ).await);
                },
                OffchainActorOutput::Output(OffchainActorOutputData::GetLastKnownBlock(maybe_last_known_block)) => {
                    match maybe_last_known_block {
                        Ok(last_known_block) => {
                            blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                                BlockchainActorInput::subscribe_finalized_blocks(last_known_block),
                                &mut released_blockchain_actor_queue
                            ).await);
                        },
                        Err(e) => {
                            log::error!("{:?}", e);
                            reset!(offchain_actor_task_queue, released_offchain_actor_queue);
                        },
                    }
                },
            }
        },
        Some(subscription_task_result) = buffered_subscription_task_queue.next() => {
            let (maybe_finalized_block_header, subscription, buf) = subscription_task_result;
            println!("BUFFERED SUBSCRIPTION TASK QUEUE");
            match maybe_finalized_block_header {
                Ok(Some(finalized_block_header)) => {
                    // Put subscription item into buffer:
                    println!("PUT SUBSCRIPTION ITEM INTO BUFFER");
                    // let finalized_block_header: <RuntimeT as System>::Header = finalized_block_header;
                    let buf: SubscriptionBufferIn = buf;
                    buf.push(finalized_block_header);
                    buffered_subscription_task_queue.push(subscription_task(subscription, buf));
                },
                err => {
                    match err {
                        Ok(Some(_)) => unreachable!(),
                        Ok(None) => { log::error!("Subscription termination unexpected"); },
                        Err(e) => { log::error!("{}", e); },
                    }
                    reset_blockchain_actor!(rpc_client_builder_actor_task_queue, released_rpc_client_builder_actor_queue);
                },
            }
        },
        Some(subscription_task_result) = subscription_task_queue.next() => {
            println!("NEXT SUBSCRIPTION TASK");
            let (finalized_block_header, buf) = subscription_task_result;
            let finalized_block_header: <RuntimeT as System>::Header = finalized_block_header;
            println!("GOT FINALIZED BLOCK HEADER");
            blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                BlockchainActorInput::get_block_events2(finalized_block_header.hash(), buf),
                &mut released_blockchain_actor_queue
            ).await);
        },
        Some(blockchain_actor_task_result) = blockchain_actor_task_queue.next() => {
            let (maybe_output, io) = blockchain_actor_task_result;
            release_actor(io, &mut released_blockchain_actor_queue).await;
            let output = if maybe_output.is_none() { unreachable!(); } else { maybe_output.unwrap() };
            match output {
                BlockchainActorOutput::Ok(BlockchainActorOutputData::GetBlockHash(_maybe_hash)) => {
                    unreachable!();
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::GetBlock(_maybe_block)) => {
                    unreachable!();
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::GetBlockEvents(_)) => {
                    unreachable!();
                },
                BlockchainActorOutput::NoClient(_input) => {
                    reset_blockchain_actor!(rpc_client_builder_actor_task_queue, released_rpc_client_builder_actor_queue);
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::SetClient) => {
                    reset!(offchain_actor_task_queue, released_offchain_actor_queue);
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::ReplayBlocks(replay)) => {
                    blocks_replay_task_queue.push(blocks_replay_task(replay));
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::SubscribeFinalizedBlocks(maybe_subscription, last_known_block, buf)) => {
                    match maybe_subscription {
                        Ok(subscription) => {
                            let buf_in = buf.detach_in();
                            let (maybe_head_block, subscription, buf_in) = subscription_task(subscription, buf_in).await;
                            match maybe_head_block {
                                Ok(Some(head_block)) => {
                                    blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                                        BlockchainActorInput::replay_blocks(last_known_block, head_block.hash(), buf),
                                        &mut released_blockchain_actor_queue
                                    ).await);
                                    // Accumulate subscription items in the buffer until blocks replay ends:
                                    println!("ACCUMULATE SUBSCRIPTION ITEMS");
                                    buffered_subscription_task_queue.push(subscription_task(subscription, buf_in));
                                },
                                err => {
                                    match err {
                                        Ok(Some(_)) => unreachable!(),
                                        Ok(None) => { log::error!("Subscription termination unexpected"); },
                                        Err(e) => { log::error!("{}", e); },
                                    }
                                    reset_blockchain_actor!(rpc_client_builder_actor_task_queue, released_rpc_client_builder_actor_queue);
                                },
                            }
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            reset_blockchain_actor!(rpc_client_builder_actor_task_queue, released_rpc_client_builder_actor_queue);
                        },
                    }
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::GetBlockEvents2(maybe_events, buf)) => {
                    println!("GET BLOCK EVENTS: {:?}", &maybe_events);
                    match maybe_events {
                        Ok(events) => {
                            let events = events.expect("EXISTENT BLOCK");
                            println!("EVENTS: {:?}", &events);
                            for x in events.into_iter() {
                                message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                                    MessageBrokerActorInput::Input(x),
                                    &mut released_message_broker_actor_queue
                                ).await);
                            }
                            // Process the next finalized block:
                            println!("SEND NEXT SUBSCRIPTION TASK");
                            subscription_task_queue.push(subscription_buffer_task(buf));
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            reset_blockchain_actor!(rpc_client_builder_actor_task_queue, released_rpc_client_builder_actor_queue);
                        },
                    }
                },
                BlockchainActorOutput::Ok(BlockchainActorOutputData::GetReplayedBlockEvents(maybe_events, replay)) => {
                    match maybe_events {
                        Ok(events) => {
                            let events = events.expect("EXISTENT BLOCK");
                            println!("REPLAYED EVENTS: {:?}", &events);
                            for x in events.into_iter() {
                                message_broker_actor_task_queue.push(init_actor_task::<_, _, MessageBrokerActorIO>(
                                    MessageBrokerActorInput::Input(x),
                                    &mut released_message_broker_actor_queue
                                ).await);
                            }
                            blocks_replay_task_queue.push(blocks_replay_task(replay));
                        },
                        Err(e) => {
                            log::error!("{}", e);
                            reset_blockchain_actor!(rpc_client_builder_actor_task_queue, released_rpc_client_builder_actor_queue);
                        },
                    }
                },
            }
        },
        Some(blocks_replay_task_result) = blocks_replay_task_queue.next() => {
            let (replay, maybe_header) = blocks_replay_task_result;
            if let Some(header) = maybe_header {
                blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                    BlockchainActorInput::get_replayed_block_events(header.hash(), replay),
                    &mut released_blockchain_actor_queue
                ).await);
            } else {
                // End of blocks replay. Just start consume items from subscription buffer:
                println!("END OF BLOCKS REPLAY");
                subscription_task_queue.push(subscription_buffer_task(replay.2));
            }
        },
        Some(message_broker_actor_task_result) = message_broker_actor_task_queue.next() => {
            let (output, io) = message_broker_actor_task_result;
            release_actor(io, &mut released_message_broker_actor_queue).await;
            log::debug!("DELIVERY STATUS: {:?}", output);
        },
        Some(rpc_client_builder_actor_task_result) = rpc_client_builder_actor_task_queue.next() => {
            let (output, io) = rpc_client_builder_actor_task_result;
            release_actor(io, &mut released_rpc_client_builder_actor_queue).await;
            match output {
                Some(Ok(client)) => {
                    blockchain_actor_task_queue.push(init_actor_task::<_, _, BlockchainActorIO>(
                        BlockchainActorInput::set_client(client),
                        &mut released_blockchain_actor_queue
                    ).await);
                },
                Some(Err(e)) => {
                    log::error!("{}", e);
                    tokio::time::sleep(Duration::from_secs(5)).await;
                    rpc_client_builder_actor_task_queue.push(init_actor_task::<_, _, RpcClientBuilderActorIO>(
                        RpcClientBuilderActorInput::Input(()),
                        &mut released_rpc_client_builder_actor_queue
                    ).await);
                },
                None => { unreachable!(); }
            }
        },
    }; }
}

#[derive(Clone)]
pub struct BufferIn<T>(mpsc::UnboundedSender<T>);
impl<T> BufferIn<T> {
    fn push(&self, item: T) {
        if self.0.send(item).is_err() { panic!("NEVER GONE"); }
    }
}
pub struct Buffer<T>(BufferIn<T>, mpsc::UnboundedReceiver<T>);
impl<T> Buffer<T> {
    fn new() -> Self {
        let (i, o) = mpsc::unbounded_channel::<T>();
        Self(BufferIn(i), o)
    }
    fn detach_in(&self) -> BufferIn<T> where T: Clone { self.0.clone() }
    
    fn push(&self, item: T) {
        self.0.push(item);
    }
    async fn pop(&mut self) -> T {
        if let Some(item) = self.1.recv().await { item } else { panic!("NEVER GONE"); }
    }
}

type ReleasedActorQueue<T> = (mpsc::Sender<T>, mpsc::Receiver<T>);

fn released_actor_queue<I, O, IO: ActorIO<I, O>>() -> ReleasedActorQueue<IO::Pair> { mpsc::channel(1) }

async fn release_actor<T>(io: T, q: &mut ReleasedActorQueue<T>) {
    if q.0.send(io).await.is_err() {
        panic!("NEVER GONE");
    }
}

async fn wait_released_actor<T>(q: &mut ReleasedActorQueue<T>) -> T {
    match q.1.recv().await {
        Some(x) => x,
        _ => panic!("NEVER GONE"),
    }
} 

type FinalizedBlocksSubscriptionItem = Result<Option<<RuntimeT as System>::Header>, jsonrpsee_ws_client::Error>;
type SubscriptionBufferIn = BufferIn<<RuntimeT as System>::Header>;

async fn subscription_task(mut subscription: FinalizedBlocksSubscription, buf: SubscriptionBufferIn)
    -> (FinalizedBlocksSubscriptionItem, FinalizedBlocksSubscription, SubscriptionBufferIn)
{
    (subscription.next().await, subscription, buf)
}

async fn subscription_buffer_task<T>(mut buf: Buffer<T>) -> (T, Buffer<T>)
{
    (buf.pop().await, buf)
}

async fn blocks_replay_task(replay: BlocksReplay) -> (BlocksReplay, Option<<RuntimeT as System>::Header>)
{
    let (task, mut rx, buf) = replay;
    let maybe_header = rx.recv().await;
    ((task, rx, buf), maybe_header)
}

type ActorTaskOutput<O, IO2> = (Option<O>, IO2);

async fn actor_task<I, O, IO>(input: I, mut io: IO::Pair) -> ActorTaskOutput<O, IO::Pair>
    where 
        I: Send, O: Send, IO: ActorIO<I, O>
{
    if io.send(input).await.is_err() { return (None, io) }
    (io.recv().await, io)
}

async fn init_actor_task<I: Send, O: Send, IO: ActorIO<I, O>>(
    input: I,
    io: &mut ReleasedActorQueue<IO::Pair>
)
    -> impl Future<Output = ActorTaskOutput<O, IO::Pair>>
{
    actor_task::<I, O, IO>(input, wait_released_actor(io).await)
}
