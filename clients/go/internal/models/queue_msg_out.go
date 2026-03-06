package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type QueueMsgOut struct {
	MsgId     string             `json:"msg_id"`
	Value     []uint8            `json:"value"`
	Headers   *map[string]string `json:"headers,omitempty"`
	Timestamp time.Time          `json:"timestamp"`
}
