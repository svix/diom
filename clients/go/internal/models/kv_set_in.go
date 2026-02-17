package diom_models

// This file is @generated DO NOT EDIT

type KvSetIn struct {
	Behavior *OperationBehavior `json:"behavior,omitempty"`
	Key      string             `json:"key"`
	Ttl      *uint64            `json:"ttl,omitempty"` // Time to live in milliseconds
	Value    []uint8            `json:"value"`
}
