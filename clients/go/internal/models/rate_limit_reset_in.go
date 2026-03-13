package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitResetIn struct {
	Key    string                     `json:"key"`
	Config RateLimitTokenBucketConfig `json:"config"` // Rate limiter configuration
}
