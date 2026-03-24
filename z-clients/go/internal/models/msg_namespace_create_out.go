package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type MsgNamespaceCreateOut struct {
	Name        string      `msgpack:"name"`
	Retention   Retention   `msgpack:"retention"`
	StorageType StorageType `msgpack:"storage_type"`
	Created     time.Time   `msgpack:"created"`
	Updated     time.Time   `msgpack:"updated"`
}
