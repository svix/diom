package diom_models

// This file is @generated DO NOT EDIT

type MsgStreamCommitIn struct {
	Namespace *string `json:"namespace,omitempty"`
	Offset    uint64  `json:"offset"`
}

type MsgStreamCommitIn_ struct {
	Namespace     *string `json:"namespace,omitempty"`
	Topic         string  `json:"topic"`
	ConsumerGroup string  `json:"consumer_group"`
	Offset        uint64  `json:"offset"`
}
