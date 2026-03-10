package diom_models

// This file is @generated DO NOT EDIT

type MsgStreamSeekIn struct {
	Topic         string  `json:"topic"`
	ConsumerGroup string  `json:"consumer_group"`
	Offset        *uint64 `json:"offset,omitempty"`
	Position      *string `json:"position,omitempty"`
}
