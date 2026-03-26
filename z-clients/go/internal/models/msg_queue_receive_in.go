package diom_models

// This file is @generated DO NOT EDIT

type MsgQueueReceiveIn struct {
	Namespace       *string `msgpack:"namespace,omitempty"`
	BatchSize       *uint16 `msgpack:"batch_size,omitempty"`
	LeaseDurationMs *uint64 `msgpack:"lease_duration_ms,omitempty"`
	BatchWaitMs     *uint64 `msgpack:"batch_wait_ms,omitempty"` // Maximum time (in milliseconds) to wait for messages before returning.
}

type MsgQueueReceiveIn_ struct {
	Namespace       *string `msgpack:"namespace,omitempty"`
	Topic           string  `msgpack:"topic"`
	ConsumerGroup   string  `msgpack:"consumer_group"`
	BatchSize       *uint16 `msgpack:"batch_size,omitempty"`
	LeaseDurationMs *uint64 `msgpack:"lease_duration_ms,omitempty"`
	BatchWaitMs     *uint64 `msgpack:"batch_wait_ms,omitempty"` // Maximum time (in milliseconds) to wait for messages before returning.
}
