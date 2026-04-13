package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type QueueMsgOut struct {
	MsgId       string                `msgpack:"msg_id"`
	Value       []uint8               `msgpack:"value"`
	Headers     map[string]string     `msgpack:"headers"`
	Timestamp   uint64                `msgpack:"timestamp"`
	ScheduledAt *diom_types.Timestamp `msgpack:"scheduled_at,omitempty"`
}
