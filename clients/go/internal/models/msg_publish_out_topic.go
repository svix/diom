package diom_models

// This file is @generated DO NOT EDIT

type MsgPublishOutTopic struct {
	Topic       string `json:"topic"`
	StartOffset uint64 `json:"start_offset"`
	Offset      uint64 `json:"offset"`
}
