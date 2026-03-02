package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type IdempotencyGetNamespaceOut struct {
	Name            string    `json:"name"`
	MaxStorageBytes *uint64   `json:"max_storage_bytes,omitempty"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
}
