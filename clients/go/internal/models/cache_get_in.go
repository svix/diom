package coyote_models

// This file is @generated DO NOT EDIT

type CacheGetIn struct {
	Namespace   *string      `msgpack:"namespace,omitempty"`
	Consistency *Consistency `msgpack:"consistency,omitempty"`
}

type CacheGetIn_ struct {
	Namespace   *string      `msgpack:"namespace,omitempty"`
	Key         string       `msgpack:"key"`
	Consistency *Consistency `msgpack:"consistency,omitempty"`
}
