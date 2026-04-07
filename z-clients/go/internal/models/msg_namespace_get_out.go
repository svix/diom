package diom_models

// This file is @generated DO NOT EDIT

type MsgNamespaceGetOut struct {
	Name      string    `msgpack:"name"`
	Retention Retention `msgpack:"retention"`
	Created   uint64    `msgpack:"created"`
	Updated   uint64    `msgpack:"updated"`
}
