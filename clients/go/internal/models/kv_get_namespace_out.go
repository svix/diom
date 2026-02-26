package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type KvGetNamespaceOut struct {
	CreatedAt       time.Time   `json:"created_at"`
	MaxStorageBytes *uint64     `json:"max_storage_bytes,omitempty"`
	Name            string      `json:"name"`
	StorageType     StorageType `json:"storage_type"`
	UpdatedAt       time.Time   `json:"updated_at"`
}
