package coyote_models

// This file is @generated DO NOT EDIT

type AuthTokenExpireIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Id        string  `msgpack:"id"`
	ExpiryMs  *uint64 `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires. `None` means expire immediately.
}
