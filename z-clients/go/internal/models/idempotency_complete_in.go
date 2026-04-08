package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type IdempotencyCompleteIn struct {
	Namespace *string                 `msgpack:"namespace,omitempty"`
	Response  []uint8                 `msgpack:"response"`          // The response to cache
	Context   *map[string]string      `msgpack:"context,omitempty"` // Optional metadata to store alongside the response
	Ttl       coyote_types.DurationMs `msgpack:"ttl_ms"`            // How long to keep the idempotency response for.
}

type IdempotencyCompleteIn_ struct {
	Namespace *string                 `msgpack:"namespace,omitempty"`
	Key       string                  `msgpack:"key"`
	Response  []uint8                 `msgpack:"response"`          // The response to cache
	Context   *map[string]string      `msgpack:"context,omitempty"` // Optional metadata to store alongside the response
	Ttl       coyote_types.DurationMs `msgpack:"ttl_ms"`            // How long to keep the idempotency response for.
}
