package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenUpdateIn struct {
	Namespace    *string            `msgpack:"namespace,omitempty"`
	Id           string             `msgpack:"id"`
	Name         *string            `msgpack:"name,omitempty"`
	ExpiryMillis *uint64            `msgpack:"expiry_millis,omitempty"`
	Metadata     *map[string]string `msgpack:"metadata,omitempty"`
	Scopes       []string           `msgpack:"scopes,omitempty"`
	Enabled      *bool              `msgpack:"enabled,omitempty"`
}
