package coyote_models

// This file is @generated DO NOT EDIT

type MsgQueueAckIn struct {
	Topic  string   `json:"topic"`
	MsgIds []string `json:"msg_ids"`
}
