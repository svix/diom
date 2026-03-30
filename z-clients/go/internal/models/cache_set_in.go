package diom_models

// This file is @generated DO NOT EDIT

type CacheSetIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Value     []uint8 `msgpack:"value"`
	TtlMs     uint64  `msgpack:"ttl_ms"` // Time to live in milliseconds
}

type CacheSetIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	Value     []uint8 `msgpack:"value"`
	TtlMs     uint64  `msgpack:"ttl_ms"` // Time to live in milliseconds
}
