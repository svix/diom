package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AdminRoleOut struct {
	Id          string            `msgpack:"id"`
	Description string            `msgpack:"description"`
	Rules       []AccessRule      `msgpack:"rules"`
	Policies    []string          `msgpack:"policies"`
	Context     map[string]string `msgpack:"context"`
	Created     time.Time         `msgpack:"created"`
	Updated     time.Time         `msgpack:"updated"`
}
