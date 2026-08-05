package diom_models

// This file is @generated DO NOT EDIT

type MsgFifoConfigureOut struct {
	RetrySchedule []uint64 `msgpack:"retry_schedule"`
	DlqTopic      *string  `msgpack:"dlq_topic,omitempty"`
}
