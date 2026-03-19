package coyote_models

// This file is @generated DO NOT EDIT

type CacheSetIn struct {
	Namespace *string `json:"namespace,omitempty"`
	Value     []uint8 `json:"value"`
	Ttl       uint64  `json:"ttl"` // Time to live in milliseconds
}

type CacheSetIn_ struct {
	Namespace *string `json:"namespace,omitempty"`
	Key       string  `json:"key"`
	Value     []uint8 `json:"value"`
	Ttl       uint64  `json:"ttl"` // Time to live in milliseconds
}
