package diom_models

// This file is @generated DO NOT EDIT

type MsgTopicDescribeIn struct {
	Namespace   *string      `msgpack:"namespace,omitempty"`
	Consistency *Consistency `msgpack:"consistency,omitempty"`
}

type MsgTopicDescribeIn_ struct {
	Namespace   *string      `msgpack:"namespace,omitempty"`
	Topic       string       `msgpack:"topic"`
	Consistency *Consistency `msgpack:"consistency,omitempty"`
}
