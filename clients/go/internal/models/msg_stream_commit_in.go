package diom_models

// This file is @generated DO NOT EDIT

type MsgStreamCommitIn struct {
	ConsumerGroup string `json:"consumer_group"`
	Offset        uint64 `json:"offset"`
	Topic         string `json:"topic"`
}
