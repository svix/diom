package coyote_models

// This file is @generated DO NOT EDIT

type MsgQueueConfigureIn struct {
	Topic         string   `json:"topic"`
	ConsumerGroup string   `json:"consumer_group"`
	RetrySchedule []uint64 `json:"retry_schedule,omitempty"`
	DlqTopic      *string  `json:"dlq_topic,omitempty"`
}
