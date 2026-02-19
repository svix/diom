package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type MsgOut struct {
	Headers   map[string]string `json:"headers"`
	Id        uint64            `json:"id"`
	Payload   []uint8           `json:"payload"`
	Timestamp time.Time         `json:"timestamp"`
}
