package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"

	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type AdminAuthTokenOut struct {
	Id      string                `msgpack:"id"`
	Name    string                `msgpack:"name"`
	Created time.Time             `msgpack:"created"`
	Updated time.Time             `msgpack:"updated"`
	Expiry  *diom_types.Timestamp `msgpack:"expiry,omitempty"`
	Role    string                `msgpack:"role"`
	Enabled bool                  `msgpack:"enabled"` // Whether this token is currently enabled.
}
