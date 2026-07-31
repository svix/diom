package diom_models

// This file is @generated DO NOT EDIT

type SinkDeleteIn struct {
	Namespace     *string `msgpack:"namespace,omitempty"`
	Topic         string  `msgpack:"topic"`
	ConsumerGroup string  `msgpack:"consumer_group"`
}
