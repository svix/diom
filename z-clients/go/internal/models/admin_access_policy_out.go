package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AdminAccessPolicyOut struct {
	Id          string       `msgpack:"id"`
	Description string       `msgpack:"description"`
	Rules       []AccessRule `msgpack:"rules"`
	Created     time.Time    `msgpack:"created"`
	Updated     time.Time    `msgpack:"updated"`
}
