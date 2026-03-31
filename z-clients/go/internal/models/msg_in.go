package diom_models

// This file is @generated DO NOT EDIT

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
	// until `delay_ms` has elapsed from the time of publish.
	DelayMs *uint64 `msgpack:"delay_ms,omitempty"`
}
