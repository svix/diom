package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AuthTokenCreateOut struct {
	Id      string    `msgpack:"id"`
	Created time.Time `msgpack:"created"`
	Updated time.Time `msgpack:"updated"`
	Token   string    `msgpack:"token"`
}
