package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.svix.com/go/diom/internal/types"
)

type RateLimitGetRemainingOut struct {
	Remaining  uint64                 `msgpack:"remaining"`                // Number of tokens remaining
	RetryAfter *diom_types.DurationMs `msgpack:"retry_after_ms,omitempty"` // Milliseconds until at least one token is available (only present when remaining is 0)
}
