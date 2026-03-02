package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type CacheGetNamespaceOut struct {
	Name            string         `json:"name"`
	MaxStorageBytes *uint64        `json:"max_storage_bytes,omitempty"`
	StorageType     StorageType    `json:"storage_type"`
	EvictionPolicy  EvictionPolicy `json:"eviction_policy"`
	CreatedAt       time.Time      `json:"created_at"`
	UpdatedAt       time.Time      `json:"updated_at"`
}
