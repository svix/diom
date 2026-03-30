package coyote_models

// This file is @generated DO NOT EDIT

type KvSetIn struct {
	Namespace *string            `msgpack:"namespace,omitempty"`
	Value     []uint8            `msgpack:"value"`
	TtlMs     *uint64            `msgpack:"ttl_ms,omitempty"` // Time to live in milliseconds
	Behavior  *OperationBehavior `msgpack:"behavior,omitempty"`
	// If set, the write only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `msgpack:"version,omitempty"`
}

type KvSetIn_ struct {
	Namespace *string            `msgpack:"namespace,omitempty"`
	Key       string             `msgpack:"key"`
	Value     []uint8            `msgpack:"value"`
	TtlMs     *uint64            `msgpack:"ttl_ms,omitempty"` // Time to live in milliseconds
	Behavior  *OperationBehavior `msgpack:"behavior,omitempty"`
	// If set, the write only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `msgpack:"version,omitempty"`
}
