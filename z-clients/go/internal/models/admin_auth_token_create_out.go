package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AdminAuthTokenCreateOut struct {
	Id      string    `msgpack:"id"`
	Token   string    `msgpack:"token"`
	Created time.Time `msgpack:"created"`
	Updated time.Time `msgpack:"updated"`
}
