use crate::actor::*;
use super::actor_io::*;

use rdkafka::producer::{FutureRecord, FutureProducer, future_producer::OwnedDeliveryResult};
use rdkafka::message::{ToBytes};
use rdkafka::util::Timeout;

use crate::events::TypedEvent;
use crate::RuntimeT;


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

pub type MessageBrokerActorInputData = Result<TypedEvent<RuntimeT>, codec::Error>;
pub type MessageBrokerActorInput = ActorDirective<MessageBrokerActorInputData>;
pub type MessageBrokerActorOutput = Result<Result<OwnedDeliveryResult, codec::Error>, serde_json::Error>;
pub type MessageBrokerActorIO = ActorJack<MessageBrokerActorInput, MessageBrokerActorOutput>;
pub type MessageBrokerActorIOPair = ActorJackPair<MessageBrokerActorIO, MessageBrokerActorInput, MessageBrokerActorOutput>;

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
        if data.is_err() {
            return Ok(Err(data.err().unwrap()))
        }
        let payload = serde_json::to_string_pretty(&data.unwrap())?;
        let record = FutureRecord::to(TOPIC)
            .key(EVENTS_KEY)
            .payload(&payload);
        let delivery_result = self.producer
            .send(record, Timeout::After(std::time::Duration::from_secs(5)))
            .await;
        Ok(Ok(delivery_result))
    }
}
