package diom_models

// This file is @generated DO NOT EDIT

type KvDeleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
}

type KvDeleteIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
}
