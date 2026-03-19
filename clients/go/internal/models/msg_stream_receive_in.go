package diom_models

// This file is @generated DO NOT EDIT

type MsgStreamReceiveIn struct {
	Namespace           *string `json:"namespace,omitempty"`
	BatchSize           *uint16 `json:"batch_size,omitempty"`
	LeaseDurationMillis *uint64 `json:"lease_duration_millis,omitempty"`
}

type MsgStreamReceiveIn_ struct {
	Namespace           *string `json:"namespace,omitempty"`
	Topic               string  `json:"topic"`
	ConsumerGroup       string  `json:"consumer_group"`
	BatchSize           *uint16 `json:"batch_size,omitempty"`
	LeaseDurationMillis *uint64 `json:"lease_duration_millis,omitempty"`
}
