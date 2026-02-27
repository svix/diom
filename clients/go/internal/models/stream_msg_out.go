package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type StreamMsgOut struct {
	Headers   *map[string]string `json:"headers,omitempty"`
	Offset    uint64             `json:"offset"`
	Timestamp time.Time          `json:"timestamp"`
	Topic     string             `json:"topic"`
	Value     []uint8            `json:"value"`
}
