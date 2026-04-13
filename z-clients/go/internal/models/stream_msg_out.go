package diom_models

// This file is @generated DO NOT EDIT

type StreamMsgOut struct {
	Offset      uint64            `msgpack:"offset"`
	Topic       string            `msgpack:"topic"`
	Value       []uint8           `msgpack:"value"`
	Headers     map[string]string `msgpack:"headers"`
	Timestamp   uint64            `msgpack:"timestamp"`
	ScheduledAt *uint64           `msgpack:"scheduled_at,omitempty"`
}
