package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type CacheGetOut struct {
	Key    string     `json:"key"`
	Expiry *time.Time `json:"expiry,omitempty"` // Time of expiry
	Value  []uint8    `json:"value"`
}
