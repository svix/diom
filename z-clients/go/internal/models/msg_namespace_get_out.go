package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type MsgNamespaceGetOut struct {
	Name      string    `msgpack:"name"`
	Retention Retention `msgpack:"retention"`
	Created   time.Time `msgpack:"created"`
	Updated   time.Time `msgpack:"updated"`
}
