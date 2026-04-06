package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitResetIn struct {
	Namespace *string         `msgpack:"namespace,omitempty"`
	Key       string          `msgpack:"key"`
	Config    RateLimitConfig `msgpack:"config"` // Rate limiter configuration
}
