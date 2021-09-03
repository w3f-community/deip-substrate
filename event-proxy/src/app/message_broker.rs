use crate::actor::*;
use super::actor_io::*;

use rdkafka::producer::{FutureRecord, FutureProducer, future_producer::OwnedDeliveryResult};
use rdkafka::util::{Timeout, TokioRuntime};
use rdkafka::client::DefaultClientContext;


pub const BOOTSTRAP_SERVERS: &str = "127.0.0.1:9092";
pub const TOPIC: &str = "blockchain";
pub const EVENTS_KEY: &str = "events";

pub struct MessageBrokerActor {
    producer: Option<FutureProducer>,
}
impl MessageBrokerActor {
    pub fn new() -> Self {
        Self { producer: None }
    }
}

pub type MessageBrokerActorInput = ActorDirective<MessageBrokerActorInputData>;
impl MessageBrokerActorInput {
    pub fn configure(config: super::KafkaConfig) -> Self {
        Self::Input(MessageBrokerActorInputData::Configure(config))
    }
    
    pub fn send_replayed_block_event(
        event: super::MaybeBlockEvent,
        remaining: usize,
        replay: super::BlocksReplay
    )
        -> Self
    {
        Self::Input(MessageBrokerActorInputData::SendReplayedBlockEvent(SendEvent {
            event,
            ctx: SendReplayedBlockEventCtx {
                remaining,
                replay,
            },
        }))
    }
    pub fn send_block_event(
        event: super::MaybeBlockEvent,
        events_buffer: super::EventsBuffer,
        remaining: usize,
        subscription_buffer: super::SubscriptionBuffer
    )
        -> Self
    {
        Self::Input(MessageBrokerActorInputData::SendBlockEvent(SendEvent {
            event,
            ctx: SendBlockEventCtx {
                events_buffer,
                remaining,
                subscription_buffer,
            },
        }))
    }
}

pub type Delivery = Result<OwnedDeliveryResult, codec::Error>;

pub struct SendEvent<Ctx> {
    event: super::MaybeBlockEvent,
    ctx: Ctx 
}
pub struct SendEventResult<Ctx> {
    delivery: Result<Delivery, serde_json::Error>,
    ctx: Ctx
}

pub struct SendReplayedBlockEventCtx {
    remaining: usize,
    replay: super::BlocksReplay,
}
pub struct SendBlockEventCtx {
    events_buffer: super::EventsBuffer,
    remaining: usize,
    subscription_buffer: super::SubscriptionBuffer,
}

pub enum MessageBrokerActorInputData {
    Configure(super::KafkaConfig),
    SendReplayedBlockEvent(SendEvent<SendReplayedBlockEventCtx>),
    SendBlockEvent(SendEvent<SendBlockEventCtx>),
}

pub enum MessageBrokerActorOutput {
    NotConfigured,
    Result(MessageBrokerActorOutputData)
}

pub enum  MessageBrokerActorOutputData {
    Configure(Option<rdkafka::error::KafkaError>),
    SendReplayedBlockEvent(SendEventResult<SendReplayedBlockEventCtx>),
    SendBlockEvent(SendEventResult<SendBlockEventCtx>),
}
impl From<MessageBrokerActorOutputData> for MessageBrokerActorOutput {
    fn from(data: MessageBrokerActorOutputData) -> Self {
        Self::Result(data)
    }
}

pub type MessageBrokerActorIO = ActorJack<MessageBrokerActorInput, MessageBrokerActorOutput>;

fn configure(c: super::KafkaConfig) -> (Option<FutureProducer>, MessageBrokerActorOutput)
{
    let mut config = rdkafka::ClientConfig::new();
    config.set("bootstrap.servers", &c.bootstrap_servers);

    match config.create::<FutureProducer>() {
        Ok(producer) => {
            let output = MessageBrokerActorOutput::Result(
                MessageBrokerActorOutputData::Configure(None)
            );
            (Some(producer), output)
        },
        Err(e) => {
            let output = MessageBrokerActorOutput::Result(
                MessageBrokerActorOutputData::Configure(Some(e))
            );
            (None, output)
        },
    }
}

#[async_trait::async_trait]
impl Actor
<
    MessageBrokerActorInputData,
    MessageBrokerActorInput,
    MessageBrokerActorOutput,
    MessageBrokerActorIO
>
for MessageBrokerActor
{
    async fn on_input(&mut self, data: MessageBrokerActorInputData) -> MessageBrokerActorOutput
    {
        if let MessageBrokerActorInputData::Configure(c) = data {
            let (producer, output) = configure(c);
            self.producer = producer;
            return output;
        }
        
        if self.producer.is_none() {
            return MessageBrokerActorOutput::NotConfigured
        }
        
        let producer = self.producer.as_ref().unwrap();
        match data {
            MessageBrokerActorInputData::Configure(_) => {
                unreachable!();
            },
            MessageBrokerActorInputData::SendReplayedBlockEvent(SendEvent { event, ctx }) => {
                if event.is_err() {
                    return MessageBrokerActorOutputData::SendReplayedBlockEvent(
                        SendEventResult { delivery: Ok(Err(event.err().unwrap())), ctx }
                    ).into();
                }
                let payload = serde_json::to_string_pretty(&event.unwrap());
                if payload.is_err() {
                    return MessageBrokerActorOutputData::SendReplayedBlockEvent(
                        SendEventResult { delivery: Err(payload.err().unwrap()), ctx }
                    ).into();
                }
                let record = FutureRecord::to(TOPIC)
                    .key(EVENTS_KEY)
                    .payload(&payload);
                let delivery_result = producer
                    .send(record, Timeout::After(std::time::Duration::from_secs(5)))
                    .await;
                MessageBrokerActorOutputData::SendReplayedBlockEvent(
                    SendEventResult { delivery: Ok(Ok(delivery_result)), ctx }
                ).into()
            },
            MessageBrokerActorInputData::SendBlockEvent { event, events_buffer, remaining, subscription_buffer } => {
                if event.is_err() {
                    return Ok(MessageBrokerActorOutputData::SendBlockEvent {
                        delivery: Err(event.err().unwrap()),
                        events_buffer,
                        remaining,
                        subscription_buffer
                    })
                }
                let payload = serde_json::to_string_pretty(&event.unwrap())?;
                let record = FutureRecord::to(TOPIC)
                    .key(EVENTS_KEY)
                    .payload(&payload);
                let delivery_result = self.producer
                    .send(record, Timeout::After(std::time::Duration::from_secs(5)))
                    .await;
                Ok(MessageBrokerActorOutputData::SendBlockEvent {
                    delivery: Ok(delivery_result),
                    events_buffer,
                    remaining,
                    subscription_buffer
                })
            },
        }
    }
}
