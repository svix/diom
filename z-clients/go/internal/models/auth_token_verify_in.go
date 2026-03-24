package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenVerifyIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Token     string  `msgpack:"token"`
}
