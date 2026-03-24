package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenCreateIn struct {
	Namespace *string            `msgpack:"namespace,omitempty"`
	Name      string             `msgpack:"name"`
	Prefix    *string            `msgpack:"prefix,omitempty"`
	Suffix    *string            `msgpack:"suffix,omitempty"`
	ExpiryMs  *uint64            `msgpack:"expiry_ms,omitempty"` // Milliseconds from now until the token expires.
	Metadata  *map[string]string `msgpack:"metadata,omitempty"`
	OwnerId   string             `msgpack:"owner_id"`
	Scopes    []string           `msgpack:"scopes,omitempty"`
	Enabled   *bool              `msgpack:"enabled,omitempty"` // Whether the token is enabled. Defaults to `true`.
}
