package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AuthTokenCreateOut struct {
	Id      string    `json:"id"`
	Created time.Time `json:"created"`
	Updated time.Time `json:"updated"`
	Token   string    `json:"token"`
}
