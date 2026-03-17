package diom_models

// This file is @generated DO NOT EDIT

type KvSetIn struct {
	Value    []uint8            `json:"value"`
	Ttl      *uint64            `json:"ttl,omitempty"` // Time to live in milliseconds
	Behavior *OperationBehavior `json:"behavior,omitempty"`
	// If set, the write only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `json:"version,omitempty"`
}

type KvSetIn_ struct {
	Key      string             `json:"key"`
	Value    []uint8            `json:"value"`
	Ttl      *uint64            `json:"ttl,omitempty"` // Time to live in milliseconds
	Behavior *OperationBehavior `json:"behavior,omitempty"`
	// If set, the write only succeeds when the stored version matches this value.
	// Use the `version` field from a prior `get` response.
	Version *uint64 `json:"version,omitempty"`
}
