package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type CacheGetOut struct {
	Expiry *time.Time `msgpack:"expiry,omitempty"` // Time of expiry
	Value  []uint8    `msgpack:"value,omitempty"`
}
