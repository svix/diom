package diom_models

// This file is @generated DO NOT EDIT

type MsgStreamSeekIn struct {
	Namespace *string `json:"namespace,omitempty"`
	Offset    *uint64 `json:"offset,omitempty"`
	Position  *string `json:"position,omitempty"`
}

type MsgStreamSeekIn_ struct {
	Namespace     *string `json:"namespace,omitempty"`
	Topic         string  `json:"topic"`
	ConsumerGroup string  `json:"consumer_group"`
	Offset        *uint64 `json:"offset,omitempty"`
	Position      *string `json:"position,omitempty"`
}
