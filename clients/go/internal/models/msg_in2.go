package coyote_models

// This file is @generated DO NOT EDIT

type MsgIn2 struct {
	Headers *map[string]string `json:"headers,omitempty"`
	Key     *string            `json:"key,omitempty"` // Optional partition key. Messages with the same key are routed to the same partition.
	Value   []uint8            `json:"value"`
}
