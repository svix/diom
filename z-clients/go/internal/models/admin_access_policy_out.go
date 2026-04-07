package coyote_models

// This file is @generated DO NOT EDIT

type AdminAccessPolicyOut struct {
	Id          string       `msgpack:"id"`
	Description string       `msgpack:"description"`
	Rules       []AccessRule `msgpack:"rules"`
	Created     uint64       `msgpack:"created"`
	Updated     uint64       `msgpack:"updated"`
}
