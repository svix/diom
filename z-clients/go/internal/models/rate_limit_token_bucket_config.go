package diom_models

// This file is @generated DO NOT EDIT

type RateLimitTokenBucketConfig struct {
	Capacity         uint64  `msgpack:"capacity"`                     // Maximum capacity of the bucket
	RefillAmount     uint64  `msgpack:"refill_amount"`                // Number of tokens to add per refill interval
	RefillIntervalMs *uint64 `msgpack:"refill_interval_ms,omitempty"` // Interval in milliseconds between refills (minimum 1 millisecond)
}
