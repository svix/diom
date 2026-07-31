package diom_models

// This file is @generated DO NOT EDIT

// Configuration for an HTTP sink. The `url`, `headers`, and `body` are templates rendered
// per-message (see [`diom_core::template_str`]).
type HttpSinkConfig struct {
	Url     string             `msgpack:"url"` // Destination URL.
	Method  *HttpMethod        `msgpack:"method,omitempty"`
	Headers *map[string]string `msgpack:"headers,omitempty"`
	Body    *string            `msgpack:"body,omitempty"` // Templated request body. When absent, the raw message value bytes are sent unchanged.
}
