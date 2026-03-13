package diom_models

// This file is @generated DO NOT EDIT

type RateLimitCheckIn struct {
	Key                  string  `json:"key"`
	Tokens               *uint64 `json:"tokens,omitempty"`                 // Number of tokens to consume (default: 1)
	Capacity             uint64  `json:"capacity"`                         // Maximum capacity of the bucket
	RefillAmount         uint64  `json:"refill_amount"`                    // Number of tokens to add per refill interval
	RefillIntervalMillis *uint64 `json:"refill_interval_millis,omitempty"` // Interval in milliseconds between refills (minimum 1 millisecond)
}
