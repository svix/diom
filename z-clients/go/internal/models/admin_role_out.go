package coyote_models

// This file is @generated DO NOT EDIT

type AdminRoleOut struct {
	Id          string            `msgpack:"id"`
	Description string            `msgpack:"description"`
	Rules       []AccessRule      `msgpack:"rules"`
	Policies    []string          `msgpack:"policies"`
	Context     map[string]string `msgpack:"context"`
	Created     uint64            `msgpack:"created"`
	Updated     uint64            `msgpack:"updated"`
}
