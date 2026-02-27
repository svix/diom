package diom_models

// This file is @generated DO NOT EDIT

type StreamCommitIn struct {
	ConsumerGroup string `json:"consumer_group"`
	Name          string `json:"name"`
	Offset        uint64 `json:"offset"`
	Topic         string `json:"topic"`
}
