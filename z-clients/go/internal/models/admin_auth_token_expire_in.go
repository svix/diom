package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type AdminAuthTokenExpireIn struct {
	Id     string                 `msgpack:"id"`
	Expiry *diom_types.DurationMs `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires. `None` means expire immediately.
}
