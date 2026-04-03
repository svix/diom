package coyote_models

// This file is @generated DO NOT EDIT

type IdempotencyCompleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Response  []uint8 `msgpack:"response"` // The response to cache
	TtlMs     uint64  `msgpack:"ttl_ms"`   // How long to keep the idempotency response for.
}

type IdempotencyCompleteIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	Response  []uint8 `msgpack:"response"` // The response to cache
	TtlMs     uint64  `msgpack:"ttl_ms"`   // How long to keep the idempotency response for.
}
