package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type RateLimitGetRemainingOut struct {
	Remaining  uint64                   `msgpack:"remaining"`                // Number of tokens remaining
	RetryAfter *coyote_types.DurationMs `msgpack:"retry_after_ms,omitempty"` // Milliseconds until at least one token is available (only present when remaining is 0)
}
