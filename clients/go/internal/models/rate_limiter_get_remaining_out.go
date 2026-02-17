package diom_models

// This file is @generated DO NOT EDIT

type RateLimiterGetRemainingOut struct {
	Remaining  uint64  `json:"remaining"`             // Number of tokens remaining
	RetryAfter *uint64 `json:"retry_after,omitempty"` // Seconds until at least one token is available (only present when remaining is 0)
}
