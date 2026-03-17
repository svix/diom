package diom_models

// This file is @generated DO NOT EDIT

type MsgQueueAckIn struct {
	MsgIds []string `json:"msg_ids"`
}

type MsgQueueAckIn_ struct {
	Topic         string   `json:"topic"`
	ConsumerGroup string   `json:"consumer_group"`
	MsgIds        []string `json:"msg_ids"`
}
