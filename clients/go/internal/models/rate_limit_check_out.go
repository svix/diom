package diom_models

// This file is @generated DO NOT EDIT

type RateLimitCheckOut struct {
	Allowed      bool    `msgpack:"allowed"`                  // Whether the request is allowed
	Remaining    uint64  `msgpack:"remaining"`                // Number of tokens remaining
	RetryAfterMs *uint64 `msgpack:"retry_after_ms,omitempty"` // Milliseconds until enough tokens are available (only present when allowed is false)
}
