package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type CacheGetOut struct {
	Expiry *coyote_types.Timestamp `msgpack:"expiry,omitempty"` // Time of expiry
	Value  []uint8                 `msgpack:"value,omitempty"`
}
