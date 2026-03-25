package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type QueueMsgOut struct {
	MsgId       string             `msgpack:"msg_id"`
	Value       []uint8            `msgpack:"value"`
	Headers     *map[string]string `msgpack:"headers,omitempty"`
	Timestamp   time.Time          `msgpack:"timestamp"`
	ScheduledAt *time.Time         `msgpack:"scheduled_at,omitempty"`
}
