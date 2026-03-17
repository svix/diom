package coyote_models

// This file is @generated DO NOT EDIT

type MsgQueueNackIn struct {
	MsgIds []string `json:"msg_ids"`
}

type MsgQueueNackIn_ struct {
	Topic         string   `json:"topic"`
	ConsumerGroup string   `json:"consumer_group"`
	MsgIds        []string `json:"msg_ids"`
}
