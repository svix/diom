package coyote_models

// This file is @generated DO NOT EDIT

type MsgIn struct {
	Headers *map[string]string `json:"headers,omitempty"`
	Payload []uint8            `json:"payload"`
}
