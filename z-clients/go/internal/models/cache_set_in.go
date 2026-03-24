package coyote_models

// This file is @generated DO NOT EDIT

type CacheSetIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Value     []uint8 `msgpack:"value"`
	Ttl       uint64  `msgpack:"ttl"` // Time to live in milliseconds
}

type CacheSetIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	Value     []uint8 `msgpack:"value"`
	Ttl       uint64  `msgpack:"ttl"` // Time to live in milliseconds
}
