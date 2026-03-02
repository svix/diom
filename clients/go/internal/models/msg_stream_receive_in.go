package coyote_models

// This file is @generated DO NOT EDIT

type MsgStreamReceiveIn struct {
	Topic               string  `json:"topic"`
	ConsumerGroup       string  `json:"consumer_group"`
	BatchSize           *uint16 `json:"batch_size,omitempty"`
	LeaseDurationMillis *uint64 `json:"lease_duration_millis,omitempty"`
}
