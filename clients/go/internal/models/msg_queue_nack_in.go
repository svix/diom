package diom_models

// This file is @generated DO NOT EDIT

type MsgQueueNackIn struct {
	Namespace *string  `msgpack:"namespace,omitempty"`
	MsgIds    []string `msgpack:"msg_ids"`
}

type MsgQueueNackIn_ struct {
	Namespace     *string  `msgpack:"namespace,omitempty"`
	Topic         string   `msgpack:"topic"`
	ConsumerGroup string   `msgpack:"consumer_group"`
	MsgIds        []string `msgpack:"msg_ids"`
}
