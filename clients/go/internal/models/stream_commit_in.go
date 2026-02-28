package coyote_models

// This file is @generated DO NOT EDIT

type StreamCommitIn struct {
	ConsumerGroup string `json:"consumer_group"`
	Offset        uint64 `json:"offset"`
	Topic         string `json:"topic"`
}
