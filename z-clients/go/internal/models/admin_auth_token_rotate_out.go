package coyote_models

// This file is @generated DO NOT EDIT

type AdminAuthTokenRotateOut struct {
	Id      string `msgpack:"id"`
	Token   string `msgpack:"token"`
	Created uint64 `msgpack:"created"`
	Updated uint64 `msgpack:"updated"`
}
