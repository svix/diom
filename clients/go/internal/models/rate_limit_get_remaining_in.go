package coyote_models

// This file is @generated DO NOT EDIT

type RateLimitGetRemainingIn struct {
	Key                  string  `json:"key"`
	Capacity             uint64  `json:"capacity"`                         // Maximum capacity of the bucket
	RefillAmount         uint64  `json:"refill_amount"`                    // Number of tokens to add per refill interval
	RefillIntervalMillis *uint64 `json:"refill_interval_millis,omitempty"` // Interval in milliseconds between refills (minimum 1 millisecond)
}
