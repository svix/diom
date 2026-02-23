package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type GetStreamOut struct {
	CreatedAt              time.Time   `json:"created_at"`
	MaxByteSize            *uint64     `json:"max_byte_size,omitempty"`
	Name                   string      `json:"name"`
	RetentionPeriodSeconds *uint64     `json:"retention_period_seconds,omitempty"`
	StorageType            StorageType `json:"storage_type"`
	UpdatedAt              time.Time   `json:"updated_at"`
}
