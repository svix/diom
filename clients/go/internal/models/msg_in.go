package diom_models

// This file is @generated DO NOT EDIT

type MsgIn struct {
	Value   []uint8            `json:"value"`
	Headers *map[string]string `json:"headers,omitempty"`
	Key     *string            `json:"key,omitempty"` // Optional partition key. Messages with the same key are routed to the same partition.
}
