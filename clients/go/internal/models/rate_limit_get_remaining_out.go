package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitGetRemainingOut struct {
	Remaining        uint64  `json:"remaining"`                    // Number of tokens remaining
	RetryAfterMillis *uint64 `json:"retry_after_millis,omitempty"` // Milliseconds until at least one token is available (only present when remaining is 0)
}
