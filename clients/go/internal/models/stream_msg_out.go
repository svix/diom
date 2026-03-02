package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type StreamMsgOut struct {
	Offset    uint64             `json:"offset"`
	Topic     string             `json:"topic"`
	Value     []uint8            `json:"value"`
	Headers   *map[string]string `json:"headers,omitempty"`
	Timestamp time.Time          `json:"timestamp"`
}
