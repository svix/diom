package coyote_models

// This file is @generated DO NOT EDIT

type MsgQueueConfigureOut struct {
	RetrySchedule []uint64 `json:"retry_schedule"`
	DlqTopic      *string  `json:"dlq_topic,omitempty"`
}
