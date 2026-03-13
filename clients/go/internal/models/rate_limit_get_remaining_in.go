package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitGetRemainingIn struct {
	Key    string                     `json:"key"`
	Config RateLimitTokenBucketConfig `json:"config"` // Rate limiter configuration
}
