package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type KvGetOut struct {
	Expiry *diom_types.Timestamp `msgpack:"expiry,omitempty"` // Time of expiry
	Value  []uint8                 `msgpack:"value,omitempty"`
	// Opaque version token for optimistic concurrency control.
	// Pass as `version` in a subsequent `set` to perform a conditional write.
	Version uint64 `msgpack:"version"`
}
