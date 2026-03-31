package diom_models

// This file is @generated DO NOT EDIT

type AdminAccessPolicyUpsertIn struct {
	Id          string       `msgpack:"id"`
	Description string       `msgpack:"description"`
	Rules       []AccessRule `msgpack:"rules,omitempty"`
}
