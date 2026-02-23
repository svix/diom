package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type IdempotencyGetGroupOut struct {
	CreatedAt       time.Time `json:"created_at"`
	MaxStorageBytes *uint64   `json:"max_storage_bytes,omitempty"`
	Name            string    `json:"name"`
	UpdatedAt       time.Time `json:"updated_at"`
}
