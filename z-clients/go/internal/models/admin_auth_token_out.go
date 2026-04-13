package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type AdminAuthTokenOut struct {
	Id      string                `msgpack:"id"`
	Name    string                `msgpack:"name"`
	Created uint64                `msgpack:"created"`
	Updated uint64                `msgpack:"updated"`
	Expiry  *diom_types.Timestamp `msgpack:"expiry,omitempty"`
	Role    string                `msgpack:"role"`
	Enabled bool                  `msgpack:"enabled"` // Whether this token is currently enabled.
}
