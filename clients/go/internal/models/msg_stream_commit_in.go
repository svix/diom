package coyote_models

// This file is @generated DO NOT EDIT

type MsgStreamCommitIn struct {
	Topic         string `json:"topic"`
	ConsumerGroup string `json:"consumer_group"`
	Offset        uint64 `json:"offset"`
}
