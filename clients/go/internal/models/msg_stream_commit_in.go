package coyote_models

// This file is @generated DO NOT EDIT

type MsgStreamCommitIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Offset    uint64  `msgpack:"offset"`
}

type MsgStreamCommitIn_ struct {
	Namespace     *string `msgpack:"namespace,omitempty"`
	Topic         string  `msgpack:"topic"`
	ConsumerGroup string  `msgpack:"consumer_group"`
	Offset        uint64  `msgpack:"offset"`
}
