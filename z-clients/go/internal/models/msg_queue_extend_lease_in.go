package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type MsgQueueExtendLeaseIn struct {
	Namespace     *string                  `msgpack:"namespace,omitempty"`
	MsgIds        []string                 `msgpack:"msg_ids"`
	LeaseDuration *coyote_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
}

type MsgQueueExtendLeaseIn_ struct {
	Namespace     *string                  `msgpack:"namespace,omitempty"`
	Topic         string                   `msgpack:"topic"`
	ConsumerGroup string                   `msgpack:"consumer_group"`
	MsgIds        []string                 `msgpack:"msg_ids"`
	LeaseDuration *coyote_types.DurationMs `msgpack:"lease_duration_ms,omitempty"`
}
