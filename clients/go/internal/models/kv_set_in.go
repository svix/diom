package diom_models

// This file is @generated DO NOT EDIT

type KvSetIn struct {
	Key      string             `json:"key"`
	Value    []uint8            `json:"value"`
	Ttl      *uint64            `json:"ttl,omitempty"` // Time to live in milliseconds
	Behavior *OperationBehavior `json:"behavior,omitempty"`
}
