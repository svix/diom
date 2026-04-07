package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type QueueMsgOut struct {
	MsgId       string                  `msgpack:"msg_id"`
	Value       []uint8                 `msgpack:"value"`
	Headers     *map[string]string      `msgpack:"headers,omitempty"`
	Timestamp   time.Time               `msgpack:"timestamp"`
	ScheduledAt *coyote_types.Timestamp `msgpack:"scheduled_at,omitempty"`
}
