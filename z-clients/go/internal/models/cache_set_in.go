package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type CacheSetIn struct {
	Namespace *string                 `msgpack:"namespace,omitempty"`
	Ttl       diom_types.DurationMs `msgpack:"ttl_ms"` // Time to live in milliseconds
}

type CacheSetIn_ struct {
	Namespace *string                 `msgpack:"namespace,omitempty"`
	Key       string                  `msgpack:"key"`
	Value     []uint8                 `msgpack:"value"`
	Ttl       diom_types.DurationMs `msgpack:"ttl_ms"` // Time to live in milliseconds
}
