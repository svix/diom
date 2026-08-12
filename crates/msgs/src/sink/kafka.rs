use diom_core::template_str::CompiledTemplate;
use rdkafka::{
    ClientConfig,
    message::{Header, OwnedHeaders},
    producer::{FutureProducer, FutureRecord},
    util::Timeout,
};

use super::{SINK_TIMEOUT, build_vars};
use crate::{entities::KafkaSinkConfig, operations::StreamReceiveMsg};

/// A Kafka sink's producer and templates compiled once. The message value and headers pass through
/// by default, with optional per-message templating.
pub(super) struct CompiledKafkaSink<'a> {
    producer: FutureProducer,
    topic: &'a str,
    key: Option<CompiledTemplate<'a>>,
    value: Option<CompiledTemplate<'a>>,
    headers: Vec<(CompiledTemplate<'a>, CompiledTemplate<'a>)>,
}

impl<'a> CompiledKafkaSink<'a> {
    pub(super) fn new(config: &'a KafkaSinkConfig) -> Result<Self, String> {
        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &config.bootstrap_servers)
            .set("message.timeout.ms", (SINK_TIMEOUT.as_millis()).to_string());
        for (key, value) in config.security.librdkafka_options() {
            client_config.set(key, value);
        }
        let producer: FutureProducer = client_config
            .create()
            .map_err(|e| format!("failed to create kafka producer: {e}"))?;
        let headers = config
            .headers
            .iter()
            .map(|(name, value)| (name.compile(), value.compile()))
            .collect();
        Ok(Self {
            producer,
            topic: &config.topic,
            key: config.key.as_ref().map(|t| t.compile()),
            value: config.value.as_ref().map(|t| t.compile()),
            headers,
        })
    }

    pub(super) async fn deliver(&self, msg: &StreamReceiveMsg) -> Result<(), String> {
        let vars = build_vars(msg);

        // Pass the message's own headers through, then merge the templated headers on top.
        let mut headers = msg.headers.clone();
        for (name, value) in &self.headers {
            headers.insert(name.apply(&vars), value.apply(&vars));
        }
        let mut record_headers = OwnedHeaders::new();
        for (name, value) in &headers {
            record_headers = record_headers.insert(Header {
                key: name,
                value: Some(value.as_bytes()),
            });
        }

        let key = self.key.as_ref().map(|t| t.apply(&vars));
        let value: Vec<u8> = match &self.value {
            Some(template) => template.apply(&vars).into_bytes(),
            None => msg.value.to_vec(),
        };

        let record = FutureRecord::to(self.topic)
            .payload(&value)
            .headers(record_headers);
        // The key branch changes the record's type parameter, so each arm sends independently.
        // Bound the enqueue wait so a full producer queue cannot block the delivery task forever.
        let timeout = Timeout::After(SINK_TIMEOUT);
        let sent = match &key {
            Some(key) => self.producer.send(record.key(key), timeout).await,
            None => self.producer.send(record, timeout).await,
        };
        sent.map(|_| ())
            .map_err(|(e, _)| format!("kafka send failed: {e}"))
    }
}
