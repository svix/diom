package coyote_models

// This file is @generated DO NOT EDIT

type IdempotencyCompleteIn struct {
	Key      string  `json:"key"`
	Response []uint8 `json:"response"` // The response to cache
	Ttl      uint64  `json:"ttl"`      // TTL in seconds for the cached response
}
