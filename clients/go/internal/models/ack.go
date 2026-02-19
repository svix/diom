package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Ack struct {
	ConsumerGroup string `json:"consumer_group"`
MsgId uint64 `json:"msg_id"`
Name string `json:"name"`
}
