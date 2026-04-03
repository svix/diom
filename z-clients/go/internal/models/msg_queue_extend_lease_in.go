package diom_models

// This file is @generated DO NOT EDIT

type MsgQueueExtendLeaseIn struct {
	Namespace       *string  `msgpack:"namespace,omitempty"`
	MsgIds          []string `msgpack:"msg_ids"`
	LeaseDurationMs *uint64  `msgpack:"lease_duration_ms,omitempty"`
}

type MsgQueueExtendLeaseIn_ struct {
	Namespace       *string  `msgpack:"namespace,omitempty"`
	Topic           string   `msgpack:"topic"`
	ConsumerGroup   string   `msgpack:"consumer_group"`
	MsgIds          []string `msgpack:"msg_ids"`
	LeaseDurationMs *uint64  `msgpack:"lease_duration_ms,omitempty"`
}
