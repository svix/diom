package diom_models

// This file is @generated DO NOT EDIT

type RateLimitResetIn struct {
	Namespace *string                    `json:"namespace,omitempty"`
	Key       string                     `json:"key"`
	Config    RateLimitTokenBucketConfig `json:"config"` // Rate limiter configuration
}
