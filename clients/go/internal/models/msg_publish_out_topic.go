package diom_models

// This file is @generated DO NOT EDIT

type MsgPublishOutTopic struct {
	Topic       string `msgpack:"topic"`
	StartOffset uint64 `msgpack:"start_offset"`
	Offset      uint64 `msgpack:"offset"`
}
