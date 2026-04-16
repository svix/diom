package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.com/go/diom/internal/types"
)

type MsgIn struct {
	Value   []uint8            `msgpack:"value"`
	Headers *map[string]string `msgpack:"headers,omitempty"`
	// Optional partition key.
	//
	// Messages with the same key are routed to the same partition.
	Key *string `msgpack:"key,omitempty"`
	// Optional delay in milliseconds.
	//
	// The message will not be delivered to queue consumers
	// until the delay has elapsed from the time of publish.
	Delay *diom_types.DurationMs `msgpack:"delay_ms,omitempty"`
}
