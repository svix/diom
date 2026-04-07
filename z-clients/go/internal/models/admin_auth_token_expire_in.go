package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type AdminAuthTokenExpireIn struct {
	Id     string                   `msgpack:"id"`
	Expiry *coyote_types.DurationMs `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires. `None` means expire immediately.
}
