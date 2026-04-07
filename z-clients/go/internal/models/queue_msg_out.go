package coyote_models

// This file is @generated DO NOT EDIT

type QueueMsgOut struct {
	MsgId       string             `msgpack:"msg_id"`
	Value       []uint8            `msgpack:"value"`
	Headers     *map[string]string `msgpack:"headers,omitempty"`
	Timestamp   uint64             `msgpack:"timestamp"`
	ScheduledAt *uint64            `msgpack:"scheduled_at,omitempty"`
}
