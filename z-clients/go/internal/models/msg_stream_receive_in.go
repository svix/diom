package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type MsgStreamReceiveIn struct {
	Namespace               *string                  `msgpack:"namespace,omitempty"`
	BatchSize               *uint16                  `msgpack:"batch_size,omitempty"`
	LeaseDuration           *coyote_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
	DefaultStartingPosition *SeekPosition            `msgpack:"default_starting_position,omitempty"`
	BatchWait               *coyote_types.DurationMs `msgpack:"batch_wait_ms,omitempty"` // Maximum time (in milliseconds) to wait for messages before returning.
}

type MsgStreamReceiveIn_ struct {
	Namespace               *string                  `msgpack:"namespace,omitempty"`
	Topic                   string                   `msgpack:"topic"`
	ConsumerGroup           string                   `msgpack:"consumer_group"`
	BatchSize               *uint16                  `msgpack:"batch_size,omitempty"`
	LeaseDuration           *coyote_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
	DefaultStartingPosition *SeekPosition            `msgpack:"default_starting_position,omitempty"`
	BatchWait               *coyote_types.DurationMs `msgpack:"batch_wait_ms,omitempty"` // Maximum time (in milliseconds) to wait for messages before returning.
}
