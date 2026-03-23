package coyote_models

// This file is @generated DO NOT EDIT

type MsgTopicConfigureIn struct {
	Namespace  *string `msgpack:"namespace,omitempty"`
	Partitions uint16  `msgpack:"partitions"`
}

type MsgTopicConfigureIn_ struct {
	Namespace  *string `msgpack:"namespace,omitempty"`
	Topic      string  `msgpack:"topic"`
	Partitions uint16  `msgpack:"partitions"`
}
