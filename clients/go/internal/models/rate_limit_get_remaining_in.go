package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitGetRemainingIn struct {
	Namespace *string                    `json:"namespace,omitempty"`
	Key       string                     `json:"key"`
	Config    RateLimitTokenBucketConfig `json:"config"` // Rate limiter configuration
}
