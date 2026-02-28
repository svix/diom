package diom_models

// This file is @generated DO NOT EDIT

type StreamReceiveIn struct {
	BatchSize           *uint16 `json:"batch_size,omitempty"`
	ConsumerGroup       string  `json:"consumer_group"`
	LeaseDurationMillis *uint64 `json:"lease_duration_millis,omitempty"`
	Topic               string  `json:"topic"`
}
