package diom_models

// This file is @generated DO NOT EDIT

type IdempotencyStartIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Ttl       uint64  `msgpack:"ttl"` // TTL in seconds for the lock/response
}

type IdempotencyStartIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	Ttl       uint64  `msgpack:"ttl"` // TTL in seconds for the lock/response
}
