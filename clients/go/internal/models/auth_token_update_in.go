package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenUpdateIn struct {
	Namespace    *string            `json:"namespace,omitempty"`
	Id           string             `json:"id"`
	Name         *string            `json:"name,omitempty"`
	ExpiryMillis *uint64            `json:"expiry_millis,omitempty"`
	Metadata     *map[string]string `json:"metadata,omitempty"`
	Scopes       []string           `json:"scopes,omitempty"`
	Enabled      *bool              `json:"enabled,omitempty"`
}
