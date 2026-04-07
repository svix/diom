package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type IdempotencyStartIn struct {
	Namespace  *string                 `msgpack:"namespace,omitempty"`
	LockPeriod coyote_types.DurationMs `msgpack:"lock_period_ms"` // How long to hold the lock on start before releasing it.
}

type IdempotencyStartIn_ struct {
	Namespace  *string                 `msgpack:"namespace,omitempty"`
	Key        string                  `msgpack:"key"`
	LockPeriod coyote_types.DurationMs `msgpack:"lock_period_ms"` // How long to hold the lock on start before releasing it.
}
