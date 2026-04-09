package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type RateLimitConfig struct {
	Capacity       uint64                 `msgpack:"capacity"`                     // Maximum capacity of the bucket
	RefillAmount   uint64                 `msgpack:"refill_amount"`                // Number of tokens to add per refill interval
	RefillInterval *diom_types.DurationMs `msgpack:"refill_interval_ms,omitempty"` // Interval in milliseconds between refills (minimum 1 millisecond)
}
