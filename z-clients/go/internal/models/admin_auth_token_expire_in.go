package diom_models

// This file is @generated DO NOT EDIT

type AdminAuthTokenExpireIn struct {
	Id       string  `msgpack:"id"`
	ExpiryMs *uint64 `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires. `None` means expire immediately.
}
