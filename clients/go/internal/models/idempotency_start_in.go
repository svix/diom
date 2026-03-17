package diom_models

// This file is @generated DO NOT EDIT

type IdempotencyStartIn struct {
	Ttl uint64 `json:"ttl"` // TTL in seconds for the lock/response
}

type IdempotencyStartIn_ struct {
	Key string `json:"key"`
	Ttl uint64 `json:"ttl"` // TTL in seconds for the lock/response
}
