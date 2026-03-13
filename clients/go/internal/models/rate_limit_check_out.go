package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitCheckOut struct {
	Status     RateLimitStatus `json:"status"`                // Whether the request is allowed
	Remaining  uint64          `json:"remaining"`             // Number of tokens remaining
	RetryAfter *uint64         `json:"retry_after,omitempty"` // Seconds until enough tokens are available (only present when allowed is false)
}
