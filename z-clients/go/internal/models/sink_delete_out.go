package diom_models

// This file is @generated DO NOT EDIT

type SinkDeleteOut struct {
	Topic         string `msgpack:"topic"`
	ConsumerGroup string `msgpack:"consumer_group"`
	Success       bool   `msgpack:"success"`
}
