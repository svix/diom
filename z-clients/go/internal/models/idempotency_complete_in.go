package diom_models

// This file is @generated DO NOT EDIT

type IdempotencyCompleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Response  []uint8 `msgpack:"response"` // The response to cache
	Ttl       uint64  `msgpack:"ttl"`      // TTL in seconds for the cached response
}

type IdempotencyCompleteIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
	Response  []uint8 `msgpack:"response"` // The response to cache
	Ttl       uint64  `msgpack:"ttl"`      // TTL in seconds for the cached response
}
