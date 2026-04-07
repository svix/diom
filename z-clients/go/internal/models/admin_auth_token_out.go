package coyote_models

// This file is @generated DO NOT EDIT

type AdminAuthTokenOut struct {
	Id      string  `msgpack:"id"`
	Name    string  `msgpack:"name"`
	Created uint64  `msgpack:"created"`
	Updated uint64  `msgpack:"updated"`
	Expiry  *uint64 `msgpack:"expiry,omitempty"`
	Role    string  `msgpack:"role"`
	Enabled bool    `msgpack:"enabled"` // Whether this token is currently enabled.
}
