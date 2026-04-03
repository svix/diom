package coyote_models

// This file is @generated DO NOT EDIT

type KvDeleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	// If set, the delete only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `msgpack:"version,omitempty"`
}

type KvDeleteIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	// If set, the delete only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `msgpack:"version,omitempty"`
}
