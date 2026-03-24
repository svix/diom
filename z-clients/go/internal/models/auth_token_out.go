package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AuthTokenOut struct {
	Id       string            `msgpack:"id"`
	Name     string            `msgpack:"name"`
	Created  time.Time         `msgpack:"created"`
	Updated  time.Time         `msgpack:"updated"`
	Expiry   *time.Time        `msgpack:"expiry,omitempty"`
	Metadata map[string]string `msgpack:"metadata"`
	OwnerId  string            `msgpack:"owner_id"`
	Scopes   []string          `msgpack:"scopes"`
	Enabled  bool              `msgpack:"enabled"` // Whether this token is currently enabled.
}
