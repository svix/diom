package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenRotateIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Id        string  `msgpack:"id"`
	Prefix    *string `msgpack:"prefix,omitempty"`
	Suffix    *string `msgpack:"suffix,omitempty"`
	ExpiryMs  *uint64 `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the old token expires. `None` means expire immediately.
}
