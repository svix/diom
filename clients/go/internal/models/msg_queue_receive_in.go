package coyote_models

// This file is @generated DO NOT EDIT

type MsgQueueReceiveIn struct {
	BatchSize           *uint16 `json:"batch_size,omitempty"`
	LeaseDurationMillis *uint64 `json:"lease_duration_millis,omitempty"`
}

type MsgQueueReceiveIn_ struct {
	Topic               string  `json:"topic"`
	ConsumerGroup       string  `json:"consumer_group"`
	BatchSize           *uint16 `json:"batch_size,omitempty"`
	LeaseDurationMillis *uint64 `json:"lease_duration_millis,omitempty"`
}
