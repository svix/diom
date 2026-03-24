package diom_models

// This file is @generated DO NOT EDIT

type RateLimitCheckIn struct {
	Namespace *string                    `msgpack:"namespace,omitempty"`
	Key       string                     `msgpack:"key"`
	Tokens    *uint64                    `msgpack:"tokens,omitempty"` // Number of tokens to consume (default: 1)
	Config    RateLimitTokenBucketConfig `msgpack:"config"`           // Rate limiter configuration
}
