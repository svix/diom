package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.svix.com/go/diom/internal/types"
)

type AdminAuthTokenCreateIn struct {
	Name    string                 `msgpack:"name"`
	Role    string                 `msgpack:"role"`
	Expiry  *diom_types.DurationMs `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires.
	Enabled *bool                  `msgpack:"enabled,omitempty"`   // Whether the token is enabled. Defaults to `true`.
}
