package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type KvGetOut struct {
	Expiry *time.Time `json:"expiry,omitempty"` // Time of expiry
	Value  []uint8    `json:"value,omitempty"`
	// Opaque version token for optimistic concurrency control.
	// Pass as `version` in a subsequent `set` to perform a conditional write.
	Version uint64 `json:"version"`
}
