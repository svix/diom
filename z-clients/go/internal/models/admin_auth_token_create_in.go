package coyote_models

// This file is @generated DO NOT EDIT

type AdminAuthTokenCreateIn struct {
	Name     string  `msgpack:"name"`
	Role     string  `msgpack:"role"`
	ExpiryMs *uint64 `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires.
	Enabled  *bool   `msgpack:"enabled,omitempty"`   // Whether the token is enabled. Defaults to `true`.
}
