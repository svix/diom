package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type StreamMsgOut struct {
	Offset      uint64                  `msgpack:"offset"`
	Topic       string                  `msgpack:"topic"`
	Value       []uint8                 `msgpack:"value"`
	Headers     *map[string]string      `msgpack:"headers,omitempty"`
	Timestamp   time.Time               `msgpack:"timestamp"`
	ScheduledAt *coyote_types.Timestamp `msgpack:"scheduled_at,omitempty"`
}
