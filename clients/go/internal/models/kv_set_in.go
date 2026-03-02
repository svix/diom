package coyote_models

// This file is @generated DO NOT EDIT

type KvSetIn struct {
	Key      string             `json:"key"`
	Ttl      *uint64            `json:"ttl,omitempty"` // Time to live in milliseconds
	Behavior *OperationBehavior `json:"behavior,omitempty"`
	Value    []uint8            `json:"value"`
}
