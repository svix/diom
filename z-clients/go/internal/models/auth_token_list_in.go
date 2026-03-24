package coyote_models

// This file is @generated DO NOT EDIT

type AuthTokenListIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	OwnerId   string  `msgpack:"owner_id"`
}
