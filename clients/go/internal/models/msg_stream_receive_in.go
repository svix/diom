package coyote_models

// This file is @generated DO NOT EDIT

type MsgStreamReceiveIn struct {
	Namespace               *string `msgpack:"namespace,omitempty"`
	BatchSize               *uint16 `msgpack:"batch_size,omitempty"`
	LeaseDurationMs         *uint64 `msgpack:"lease_duration_ms,omitempty"`
	DefaultStartingPosition *string `msgpack:"default_starting_position,omitempty"`
}

type MsgStreamReceiveIn_ struct {
	Namespace               *string `msgpack:"namespace,omitempty"`
	Topic                   string  `msgpack:"topic"`
	ConsumerGroup           string  `msgpack:"consumer_group"`
	BatchSize               *uint16 `msgpack:"batch_size,omitempty"`
	LeaseDurationMs         *uint64 `msgpack:"lease_duration_ms,omitempty"`
	DefaultStartingPosition *string `msgpack:"default_starting_position,omitempty"`
}
