package diom_models

// This file is @generated DO NOT EDIT

type CacheSetIn struct {
	Value []uint8 `json:"value"`
	Ttl   uint64  `json:"ttl"` // Time to live in milliseconds
}

type CacheSetIn_ struct {
	Key   string  `json:"key"`
	Value []uint8 `json:"value"`
	Ttl   uint64  `json:"ttl"` // Time to live in milliseconds
}
