package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.com/go/diom/internal/types"
)

type MsgStreamSeekIn struct {
	Namespace *string               `msgpack:"namespace,omitempty"`
	Offset    *uint64               `msgpack:"offset,omitempty"`
	Position  *SeekPosition         `msgpack:"position,omitempty"`
	Timestamp *diom_types.Timestamp `msgpack:"timestamp,omitempty"`
}

type MsgStreamSeekIn_ struct {
	Namespace     *string               `msgpack:"namespace,omitempty"`
	Topic         string                `msgpack:"topic"`
	ConsumerGroup string                `msgpack:"consumer_group"`
	Offset        *uint64               `msgpack:"offset,omitempty"`
	Position      *SeekPosition         `msgpack:"position,omitempty"`
	Timestamp     *diom_types.Timestamp `msgpack:"timestamp,omitempty"`
}
