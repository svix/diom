package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type CacheSetIn struct {
	Namespace *string                 `msgpack:"namespace,omitempty"`
	Ttl       coyote_types.DurationMs `msgpack:"ttl_ms"` // Time to live in milliseconds
}

type CacheSetIn_ struct {
	Namespace *string                 `msgpack:"namespace,omitempty"`
	Key       string                  `msgpack:"key"`
	Value     []uint8                 `msgpack:"value"`
	Ttl       coyote_types.DurationMs `msgpack:"ttl_ms"` // Time to live in milliseconds
}
