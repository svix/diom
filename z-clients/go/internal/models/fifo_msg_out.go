package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.com/go/diom/internal/types"
)

type FifoMsgOut struct {
	MsgId       string                `msgpack:"msg_id"`
	Key         *string               `msgpack:"key,omitempty"`
	Value       []uint8               `msgpack:"value"`
	Headers     map[string]string     `msgpack:"headers"`
	Timestamp   uint64                `msgpack:"timestamp"`
	ScheduledAt *diom_types.Timestamp `msgpack:"scheduled_at,omitempty"`
}
