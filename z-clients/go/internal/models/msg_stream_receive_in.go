package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.svix.com/go/diom/internal/types"
)

type MsgStreamReceiveIn struct {
	Namespace               *string                `msgpack:"namespace,omitempty"`
	BatchSize               *uint16                `msgpack:"batch_size,omitempty"`
	LeaseDuration           *diom_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
	DefaultStartingPosition *SeekPosition          `msgpack:"default_starting_position,omitempty"`
	BatchWait               *diom_types.DurationMs `msgpack:"batch_wait_ms,omitempty"` // Maximum time (in milliseconds) to wait for messages before returning.
}

type MsgStreamReceiveIn_ struct {
	Namespace               *string                `msgpack:"namespace,omitempty"`
	Topic                   string                 `msgpack:"topic"`
	ConsumerGroup           string                 `msgpack:"consumer_group"`
	BatchSize               *uint16                `msgpack:"batch_size,omitempty"`
	LeaseDuration           *diom_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
	DefaultStartingPosition *SeekPosition          `msgpack:"default_starting_position,omitempty"`
	BatchWait               *diom_types.DurationMs `msgpack:"batch_wait_ms,omitempty"` // Maximum time (in milliseconds) to wait for messages before returning.
}
