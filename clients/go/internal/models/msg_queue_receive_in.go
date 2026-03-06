package diom_models

// This file is @generated DO NOT EDIT

type MsgQueueReceiveIn struct {
	Topic               string  `json:"topic"`
	BatchSize           *uint16 `json:"batch_size,omitempty"`
	LeaseDurationMillis *uint64 `json:"lease_duration_millis,omitempty"`
}
