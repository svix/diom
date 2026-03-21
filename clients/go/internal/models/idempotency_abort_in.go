package coyote_models

// This file is @generated DO NOT EDIT

type IdempotencyAbortIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
}

type IdempotencyAbortIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Key       string  `msgpack:"key"`
}
