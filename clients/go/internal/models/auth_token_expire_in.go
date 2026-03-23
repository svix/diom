package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenExpireIn struct {
	Namespace    *string `msgpack:"namespace,omitempty"`
	Id           string  `msgpack:"id"`
	ExpiryMillis *uint64 `msgpack:"expiry_millis,omitempty"` // Milliseconds from now until the token expires. `None` means expire immediately.
}
