package diom_models

// This file is @generated DO NOT EDIT

type AdminAuthTokenUpdateIn struct {
	Id       string  `msgpack:"id"`
	Name     *string `msgpack:"name,omitempty"`
	ExpiryMs *uint64 `msgpack:"expiry_ms,omitempty"`
	Enabled  *bool   `msgpack:"enabled,omitempty"`
}
