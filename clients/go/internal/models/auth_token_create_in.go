package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenCreateIn struct {
	Namespace    *string            `json:"namespace,omitempty"`
	Name         string             `json:"name"`
	Prefix       *string            `json:"prefix,omitempty"`
	Suffix       *string            `json:"suffix,omitempty"`
	ExpiryMillis *uint64            `json:"expiry_millis,omitempty"` // Milliseconds from now until the token expires.
	Metadata     *map[string]string `json:"metadata,omitempty"`
	OwnerId      string             `json:"owner_id"`
	Scopes       []string           `json:"scopes,omitempty"`
	Enabled      *bool              `json:"enabled,omitempty"` // Whether the token is enabled. Defaults to `true`.
}
