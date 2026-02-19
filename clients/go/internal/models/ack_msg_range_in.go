package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type AckMsgRangeIn struct {
	ConsumerGroup string `json:"consumer_group"`
MaxMsgId uint64 `json:"max_msg_id"`
MinMsgId *uint64 `json:"min_msg_id,omitempty"`
Name string `json:"name"`
}
