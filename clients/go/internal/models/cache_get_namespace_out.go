package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type CacheGetNamespaceOut struct {
	CreatedAt       time.Time      `json:"created_at"`
	EvictionPolicy  EvictionPolicy `json:"eviction_policy"`
	MaxStorageBytes *uint64        `json:"max_storage_bytes,omitempty"`
	Name            string         `json:"name"`
	StorageType     StorageType    `json:"storage_type"`
	UpdatedAt       time.Time      `json:"updated_at"`
}
