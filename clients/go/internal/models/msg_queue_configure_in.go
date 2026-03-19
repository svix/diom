package diom_models

// This file is @generated DO NOT EDIT

type MsgQueueConfigureIn struct {
	Namespace     *string  `json:"namespace,omitempty"`
	RetrySchedule []uint64 `json:"retry_schedule,omitempty"`
	DlqTopic      *string  `json:"dlq_topic,omitempty"`
}

type MsgQueueConfigureIn_ struct {
	Namespace     *string  `json:"namespace,omitempty"`
	Topic         string   `json:"topic"`
	ConsumerGroup string   `json:"consumer_group"`
	RetrySchedule []uint64 `json:"retry_schedule,omitempty"`
	DlqTopic      *string  `json:"dlq_topic,omitempty"`
}
