package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type AdminAuthTokenUpdateIn struct {
	Id      string                   `msgpack:"id"`
	Name    *string                  `msgpack:"name,omitempty"`
	Expiry  *coyote_types.DurationMs `msgpack:"expiry_ms,omitempty"`
	Enabled *bool                    `msgpack:"enabled,omitempty"`
}
