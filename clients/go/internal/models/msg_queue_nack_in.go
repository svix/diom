package diom_models

// This file is @generated DO NOT EDIT

type MsgQueueNackIn struct {
	Namespace *string  `json:"namespace,omitempty"`
	MsgIds    []string `json:"msg_ids"`
}

type MsgQueueNackIn_ struct {
	Namespace     *string  `json:"namespace,omitempty"`
	Topic         string   `json:"topic"`
	ConsumerGroup string   `json:"consumer_group"`
	MsgIds        []string `json:"msg_ids"`
}
