package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.svix.com/go/diom/internal/types"
)

type AdminAuthTokenUpdateIn struct {
	Id      string                 `msgpack:"id"`
	Name    *string                `msgpack:"name,omitempty"`
	Expiry  *diom_types.DurationMs `msgpack:"expiry_ms,omitempty"`
	Enabled *bool                  `msgpack:"enabled,omitempty"`
}
