package coyote_models

// This file is @generated DO NOT EDIT

type AccessRule struct {
	Effect   AccessRuleEffect `msgpack:"effect"`
	Resource string           `msgpack:"resource"`
	Actions  []string         `msgpack:"actions"`
}
