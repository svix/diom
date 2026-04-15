package diom_models

// This file is @generated DO NOT EDIT

type AdminRoleConfigureIn struct {
	Id          string             `msgpack:"id"`
	Description string             `msgpack:"description"`
	Rules       []AccessRule       `msgpack:"rules,omitempty"`
	Policies    []string           `msgpack:"policies,omitempty"`
	Context     *map[string]string `msgpack:"context,omitempty"`
}
