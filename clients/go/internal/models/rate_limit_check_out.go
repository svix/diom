package diom_models

// This file is @generated DO NOT EDIT

type RateLimitCheckOut struct {
	Allowed          bool    `json:"allowed"`                      // Whether the request is allowed
	Remaining        uint64  `json:"remaining"`                    // Number of tokens remaining
	RetryAfterMillis *uint64 `json:"retry_after_millis,omitempty"` // Milliseconds until enough tokens are available (only present when allowed is false)
}
