package coyote_models

// This file is @generated DO NOT EDIT

type CacheDeleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
}

type CacheDeleteIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
}
