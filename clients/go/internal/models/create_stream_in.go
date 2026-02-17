package diom_models

// This file is @generated DO NOT EDIT

type CreateStreamIn struct {
	MaxByteSize            *uint64 `json:"max_byte_size,omitempty"` // How many bytes in total the stream will retain before dropping data.
	Name                   string  `json:"name"`
	RetentionPeriodSeconds *uint64 `json:"retention_period_seconds,omitempty"` // How long messages are retained in the stream before being permanently nuked.
}
