package diom_models

// This file is @generated DO NOT EDIT

type MsgFifoConfigureIn struct {
	Namespace     *string  `msgpack:"namespace,omitempty"`
	RetrySchedule []uint64 `msgpack:"retry_schedule,omitempty"`
	DlqTopic      *string  `msgpack:"dlq_topic,omitempty"`
}

type MsgFifoConfigureIn_ struct {
	Namespace     *string  `msgpack:"namespace,omitempty"`
	Topic         string   `msgpack:"topic"`
	ConsumerGroup string   `msgpack:"consumer_group"`
	RetrySchedule []uint64 `msgpack:"retry_schedule,omitempty"`
	DlqTopic      *string  `msgpack:"dlq_topic,omitempty"`
}
