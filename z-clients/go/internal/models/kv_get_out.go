package coyote_models

// This file is @generated DO NOT EDIT

type KvGetOut struct {
	Expiry *uint64 `msgpack:"expiry,omitempty"` // Time of expiry
	Value  []uint8 `msgpack:"value,omitempty"`
	// Opaque version token for optimistic concurrency control.
	// Pass as `version` in a subsequent `set` to perform a conditional write.
	Version uint64 `msgpack:"version"`
}
