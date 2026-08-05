package diom_models

// This file is @generated DO NOT EDIT

type MsgFifoRedriveDlqIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
}

type MsgFifoRedriveDlqIn_ struct {
	Namespace     *string `msgpack:"namespace,omitempty"`
	Topic         string  `msgpack:"topic"`
	ConsumerGroup string  `msgpack:"consumer_group"`
}
