use crate::actor::*;
use super::actor_io::*;

use rdkafka::producer::{FutureRecord, FutureProducer, future_producer::OwnedDeliveryResult};
use rdkafka::util::Timeout;


pub const BOOTSTRAP_SERVERS: &str = "127.0.0.1:9092";
pub const TOPIC: &str = "blockchain";
pub const EVENTS_KEY: &str = "events";

pub struct MessageBrokerActor {
    producer: FutureProducer
}
impl MessageBrokerActor {
    pub fn new() -> Self {
        let mut config = rdkafka::ClientConfig::new();
        config.set("bootstrap.servers", BOOTSTRAP_SERVERS);
        let producer = config.create::<FutureProducer>().unwrap();
        Self { producer }
    }
}

pub type MessageBrokerActorInput = ActorDirective<MessageBrokerActorInputData>;
impl MessageBrokerActorInput {
    pub fn send_replayed_block_event(
        event: super::MaybeBlockEvent,
        remaining: usize,
        replay: super::BlocksReplay
    )
        -> Self
    {
        Self::Input(MessageBrokerActorInputData::SendReplayedBlockEvent {
            event,
            remaining,
            replay,
        })
    }
    pub fn send_block_event(
        event: super::MaybeBlockEvent,
        events_buffer: super::EventsBuffer,
        remaining: usize,
        subscription_buffer: super::SubscriptionBuffer
    )
        -> Self
    {
        Self::Input(MessageBrokerActorInputData::SendBlockEvent {
            event,
            events_buffer,
            remaining,
            subscription_buffer
        })
    }
}
pub enum MessageBrokerActorInputData {
    SendReplayedBlockEvent {
        event: super::MaybeBlockEvent,
        remaining: usize,
        replay: super::BlocksReplay
    },
    SendBlockEvent {
        event: super::MaybeBlockEvent,
        events_buffer: super::EventsBuffer,
        remaining: usize,
        subscription_buffer: super::SubscriptionBuffer
    },
}

pub type Delivery = Result<OwnedDeliveryResult, codec::Error>;

pub type MessageBrokerActorOutput = Result<MessageBrokerActorOutputData, serde_json::Error>;

pub enum  MessageBrokerActorOutputData {
    SendReplayedBlockEvent {
        delivery: Delivery,
        remaining: usize,
        replay: super::BlocksReplay
    },
    SendBlockEvent {
        delivery: Delivery,
        events_buffer: super::EventsBuffer,
        remaining: usize,
        subscription_buffer: super::SubscriptionBuffer
    }
}

pub type MessageBrokerActorIO = ActorJack<MessageBrokerActorInput, MessageBrokerActorOutput>;

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
    async fn on_input(&mut self, data: MessageBrokerActorInputData) -> MessageBrokerActorOutput {
        match data {
            MessageBrokerActorInputData::SendReplayedBlockEvent { event, remaining, replay } => {
                if event.is_err() {
                    return Ok(MessageBrokerActorOutputData::SendReplayedBlockEvent {
                        delivery: Err(event.err().unwrap()),
                        remaining,
                        replay
                    })
                }
                let payload = serde_json::to_string_pretty(&event.unwrap())?;
                let record = FutureRecord::to(TOPIC)
                    .key(EVENTS_KEY)
                    .payload(&payload);
                let delivery_result = self.producer
                    .send(record, Timeout::After(std::time::Duration::from_secs(5)))
                    .await;
                Ok(MessageBrokerActorOutputData::SendReplayedBlockEvent {
                    delivery: Ok(delivery_result),
                    remaining,
                    replay
                })
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
