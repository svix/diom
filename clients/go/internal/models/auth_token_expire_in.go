package coyote_models

// This file is @generated DO NOT EDIT

type AuthTokenExpireIn struct {
	Namespace    *string `json:"namespace,omitempty"`
	Id           string  `json:"id"`
	ExpiryMillis *uint64 `json:"expiry_millis,omitempty"` // Milliseconds from now until the token expires. `None` means expire immediately.
}
