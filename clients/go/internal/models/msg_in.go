package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type MsgIn struct {
	Headers *map[string]string `json:"headers,omitempty"`
Payload []uint8 `json:"payload"`
}
