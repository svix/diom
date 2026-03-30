package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type IdempotencyGetNamespaceOut struct {
	Name    string    `msgpack:"name"`
	Created time.Time `msgpack:"created"`
	Updated time.Time `msgpack:"updated"`
}
