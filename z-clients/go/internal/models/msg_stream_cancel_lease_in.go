package diom_models

// This file is @generated DO NOT EDIT

type MsgStreamCancelLeaseIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
}

type MsgStreamCancelLeaseIn_ struct {
	Namespace     *string `msgpack:"namespace,omitempty"`
	Topic         string  `msgpack:"topic"`
	ConsumerGroup string  `msgpack:"consumer_group"`
}
