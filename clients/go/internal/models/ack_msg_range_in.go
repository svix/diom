package coyote_models

// This file is @generated DO NOT EDIT

type AckMsgRangeIn struct {
	ConsumerGroup string  `json:"consumer_group"`
	MaxMsgId      uint64  `json:"max_msg_id"`
	MinMsgId      *uint64 `json:"min_msg_id,omitempty"`
	Name          string  `json:"name"`
}
