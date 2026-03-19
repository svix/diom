package coyote_models

// This file is @generated DO NOT EDIT

type MsgQueueRedriveDlqIn struct {
	Namespace *string `json:"namespace,omitempty"`
}

type MsgQueueRedriveDlqIn_ struct {
	Namespace     *string `json:"namespace,omitempty"`
	Topic         string  `json:"topic"`
	ConsumerGroup string  `json:"consumer_group"`
}
