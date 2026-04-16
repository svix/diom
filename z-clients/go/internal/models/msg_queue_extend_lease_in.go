package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.svix.com/go/diom/internal/types"
)

type MsgQueueExtendLeaseIn struct {
	Namespace     *string                `msgpack:"namespace,omitempty"`
	MsgIds        []string               `msgpack:"msg_ids"`
	LeaseDuration *diom_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
}

type MsgQueueExtendLeaseIn_ struct {
	Namespace     *string                `msgpack:"namespace,omitempty"`
	Topic         string                 `msgpack:"topic"`
	ConsumerGroup string                 `msgpack:"consumer_group"`
	MsgIds        []string               `msgpack:"msg_ids"`
	LeaseDuration *diom_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
}
