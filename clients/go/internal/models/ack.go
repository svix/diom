package coyote_models

// This file is @generated DO NOT EDIT

type Ack struct {
	ConsumerGroup string `json:"consumer_group"`
	MsgId         uint64 `json:"msg_id"`
	Name          string `json:"name"`
}
