package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type KvGetOut struct {
	Expiry *time.Time `json:"expiry,omitempty"` // Time of expiry
	Value  []uint8    `json:"value,omitempty"`
}
