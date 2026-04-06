package coyote_models

// This file is @generated DO NOT EDIT

type IdempotencyStartIn struct {
	Namespace    *string `msgpack:"namespace,omitempty"`
	LockPeriodMs uint64  `msgpack:"lock_period_ms"` // How long to hold the lock on start before releasing it.
}

type IdempotencyStartIn_ struct {
	Namespace    *string `msgpack:"namespace,omitempty"`
	Key          string  `msgpack:"key"`
	LockPeriodMs uint64  `msgpack:"lock_period_ms"` // How long to hold the lock on start before releasing it.
}
