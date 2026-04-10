package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type CacheGetOut struct {
	Expiry time.Time `msgpack:"expiry"` // Time of expiry
	Value  []uint8   `msgpack:"value,omitempty"`
}
