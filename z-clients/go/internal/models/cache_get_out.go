package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.com/go/diom/internal/types"
)

type CacheGetOut struct {
	Expiry *diom_types.Timestamp `msgpack:"expiry,omitempty"` // Time of expiry
	Value  []uint8               `msgpack:"value,omitempty"`
}
