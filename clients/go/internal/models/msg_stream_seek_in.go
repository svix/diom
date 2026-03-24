package coyote_models

// This file is @generated DO NOT EDIT

type MsgStreamSeekIn struct {
	Namespace *string       `msgpack:"namespace,omitempty"`
	Offset    *uint64       `msgpack:"offset,omitempty"`
	Position  *SeekPosition `msgpack:"position,omitempty"`
}

type MsgStreamSeekIn_ struct {
	Namespace     *string       `msgpack:"namespace,omitempty"`
	Topic         string        `msgpack:"topic"`
	ConsumerGroup string        `msgpack:"consumer_group"`
	Offset        *uint64       `msgpack:"offset,omitempty"`
	Position      *SeekPosition `msgpack:"position,omitempty"`
}
