package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type AuthTokenCreateOut struct {
	Id        string    `json:"id"`
	CreatedAt time.Time `json:"created_at"`
	UpdatedAt time.Time `json:"updated_at"`
	Token     string    `json:"token"`
}
