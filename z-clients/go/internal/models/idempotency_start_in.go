package diom_models

// This file is @generated DO NOT EDIT

type IdempotencyStartIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	TtlMs     uint64  `msgpack:"ttl_ms"` // TTL in milliseconds for the lock/response
}

type IdempotencyStartIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	TtlMs     uint64  `msgpack:"ttl_ms"` // TTL in milliseconds for the lock/response
}
