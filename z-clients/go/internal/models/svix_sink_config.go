package diom_models

// This file is @generated DO NOT EDIT

// Configuration for a Svix sink. Each message is forwarded as a Svix message-create call
// (`POST {server_url}/api/v1/app/{app_id}/msg/`). This is a thin convenience over an HTTP sink.
// The `app_id`, `event_type`, and `payload` are templates rendered per-message
// (see [`diom_core::template_str`]).
type SvixSinkConfig struct {
	Token     string  `msgpack:"token"`             // Svix API token, sent as the bearer credential. Obfuscated in list responses.
	AppId     string  `msgpack:"app_id"`            // Target Svix application. Can be optionally templated.
	EventType string  `msgpack:"event_type"`        // Svix event type. Can be optionally templated.
	Payload   *string `msgpack:"payload,omitempty"` // Templated message payload. When absent, the raw message value bytes are used (must be JSON).
	// Templated Svix `Idempotency-Key`. When absent or it renders to an empty string, a stable
	// key derived from the sink and message identity (namespace, topic, consumer_group, partition,
	// offset) is used so retries are de-duplicated by Svix.
	IdempotencyKey *string `msgpack:"idempotency_key,omitempty"`
	ServerUrl      *string `msgpack:"server_url,omitempty"` // Optional base URL override. When absent, the region is inferred from the token.
}
