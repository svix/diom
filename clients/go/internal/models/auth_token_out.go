package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AuthTokenOut struct {
	Id       string            `json:"id"`
	Name     string            `json:"name"`
	Created  time.Time         `json:"created"`
	Updated  time.Time         `json:"updated"`
	Expiry   *time.Time        `json:"expiry,omitempty"`
	Metadata map[string]string `json:"metadata"`
	OwnerId  string            `json:"owner_id"`
	Scopes   []string          `json:"scopes"`
	Enabled  bool              `json:"enabled"` // Whether this token is currently enabled.
}
