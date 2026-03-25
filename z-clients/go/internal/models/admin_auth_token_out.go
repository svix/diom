package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AdminAuthTokenOut struct {
	Id      string     `msgpack:"id"`
	Name    string     `msgpack:"name"`
	Created time.Time  `msgpack:"created"`
	Updated time.Time  `msgpack:"updated"`
	Expiry  *time.Time `msgpack:"expiry,omitempty"`
	Role    string     `msgpack:"role"`
	Enabled bool       `msgpack:"enabled"` // Whether this token is currently enabled.
}
