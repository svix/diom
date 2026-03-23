package diom_models

// This file is @generated DO NOT EDIT

type KvGetIn struct {
	Namespace   *string      `msgpack:"namespace,omitempty"`
	Consistency *Consistency `msgpack:"consistency,omitempty"`
}

type KvGetIn_ struct {
	Namespace   *string      `msgpack:"namespace,omitempty"`
	Key         string       `msgpack:"key"`
	Consistency *Consistency `msgpack:"consistency,omitempty"`
}
