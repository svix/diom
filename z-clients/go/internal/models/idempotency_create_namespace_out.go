package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type IdempotencyCreateNamespaceOut struct {
	Name            string    `msgpack:"name"`
	MaxStorageBytes *uint64   `msgpack:"max_storage_bytes,omitempty"`
	Created         time.Time `msgpack:"created"`
	Updated         time.Time `msgpack:"updated"`
}
