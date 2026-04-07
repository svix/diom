package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type RateLimitCheckOut struct {
	Allowed    bool                     `msgpack:"allowed"`                  // Whether the request is allowed
	Remaining  uint64                   `msgpack:"remaining"`                // Number of tokens remaining
	RetryAfter *coyote_types.DurationMs `msgpack:"retry_after_ms,omitempty"` // Milliseconds until enough tokens are available (only present when allowed is false)
}
