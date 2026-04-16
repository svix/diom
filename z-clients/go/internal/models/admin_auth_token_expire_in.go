package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "diom.com/go/diom/internal/types"
)

type AdminAuthTokenExpireIn struct {
	Id     string                 `msgpack:"id"`
	Expiry *diom_types.DurationMs `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires. `None` means expire immediately.
}
