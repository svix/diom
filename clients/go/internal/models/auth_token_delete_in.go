package coyote_models

// This file is @generated DO NOT EDIT

type AuthTokenDeleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Id        string  `msgpack:"id"`
}
