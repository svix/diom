package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type KvGetOut struct {
	Expiry *coyote_types.Timestamp `msgpack:"expiry,omitempty"` // Time of expiry
	Value  []uint8                 `msgpack:"value,omitempty"`
	// Opaque version token for optimistic concurrency control.
	// Pass as `version` in a subsequent `set` to perform a conditional write.
	Version uint64 `msgpack:"version"`
}
