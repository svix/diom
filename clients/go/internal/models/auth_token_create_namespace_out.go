package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AuthTokenCreateNamespaceOut struct {
	Name            string      `json:"name"`
	MaxStorageBytes *uint64     `json:"max_storage_bytes,omitempty"`
	StorageType     StorageType `json:"storage_type"`
	Created         time.Time   `json:"created"`
	Updated         time.Time   `json:"updated"`
}
