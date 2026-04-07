package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type AdminAuthTokenCreateIn struct {
	Name    string                   `msgpack:"name"`
	Role    string                   `msgpack:"role"`
	Expiry  *coyote_types.DurationMs `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires.
	Enabled *bool                    `msgpack:"enabled,omitempty"`   // Whether the token is enabled. Defaults to `true`.
}
