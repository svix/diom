package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenRotateIn struct {
	Namespace    *string `json:"namespace,omitempty"`
	Id           string  `json:"id"`
	Prefix       *string `json:"prefix,omitempty"`
	Suffix       *string `json:"suffix,omitempty"`
	ExpiryMillis *uint64 `json:"expiry_millis,omitempty"` // Milliseconds from now until the old token expires. `None` means expire immediately.
}
