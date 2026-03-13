package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitCheckIn struct {
	Key    string                     `json:"key"`
	Tokens *uint64                    `json:"tokens,omitempty"` // Number of tokens to consume (default: 1)
	Config RateLimitTokenBucketConfig `json:"config"`           // Rate limiter configuration
}
