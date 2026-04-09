package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type IdempotencyStartIn struct {
	Namespace  *string               `msgpack:"namespace,omitempty"`
	LockPeriod diom_types.DurationMs `msgpack:"lock_period_ms"` // How long to hold the lock on start before releasing it.
}

type IdempotencyStartIn_ struct {
	Namespace  *string               `msgpack:"namespace,omitempty"`
	Key        string                `msgpack:"key"`
	LockPeriod diom_types.DurationMs `msgpack:"lock_period_ms"` // How long to hold the lock on start before releasing it.
}
