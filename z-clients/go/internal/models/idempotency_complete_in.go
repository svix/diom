package coyote_models

// This file is @generated DO NOT EDIT

type IdempotencyCompleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Response  []uint8 `msgpack:"response"` // The response to cache
	TtlMs     uint64  `msgpack:"ttl_ms"`   // TTL in milliseconds for the cached response
}

type IdempotencyCompleteIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	Response  []uint8 `msgpack:"response"` // The response to cache
	TtlMs     uint64  `msgpack:"ttl_ms"`   // TTL in milliseconds for the cached response
}
