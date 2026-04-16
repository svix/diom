package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.com/go/diom/internal/types"
)

type KvSetIn struct {
	Namespace *string                `msgpack:"namespace,omitempty"`
	Ttl       *diom_types.DurationMs `msgpack:"ttl_ms,omitempty"` // Time to live in milliseconds
	Behavior  *OperationBehavior     `msgpack:"behavior,omitempty"`
	// If set, the write only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `msgpack:"version,omitempty"`
}

type KvSetIn_ struct {
	Namespace *string                `msgpack:"namespace,omitempty"`
	Key       string                 `msgpack:"key"`
	Value     []uint8                `msgpack:"value"`
	Ttl       *diom_types.DurationMs `msgpack:"ttl_ms,omitempty"` // Time to live in milliseconds
	Behavior  *OperationBehavior     `msgpack:"behavior,omitempty"`
	// If set, the write only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `msgpack:"version,omitempty"`
}
