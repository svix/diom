package diom_models

// This file is @generated DO NOT EDIT

// Configuration for a Kafka sink. Each message is produced to `topic` on the target cluster. By
// default the message value and headers pass through unchanged, but each can be templated
// per-message (see [`diom_core::template_str`]).
type KafkaSinkConfig struct {
	BootstrapServers string  `msgpack:"bootstrap_servers"` // Comma-separated `host:port` list of the target cluster's bootstrap brokers.
	Topic            string  `msgpack:"topic"`             // Destination Kafka topic.
	Key              *string `msgpack:"key,omitempty"`     // Templated record key rendered per-message. When absent, records are produced without a key.
	Value            *string `msgpack:"value,omitempty"`   // Templated record value. When absent, the raw message value bytes are produced unchanged.
	// Templated record headers merged on top of the message's own headers (which pass through by
	// default). A templated header overrides a passed-through one with the same name.
	Headers  *map[string]string `msgpack:"headers,omitempty"`
	Security *KafkaSecurity     `msgpack:"security,omitempty"` // Connection security (SASL and/or TLS). Defaults to none (PLAINTEXT).
}
