package coyote_models

// This file is @generated DO NOT EDIT

type CacheGetIn struct {
	Namespace   *string      `json:"namespace,omitempty"`
	Consistency *Consistency `json:"consistency,omitempty"`
}

type CacheGetIn_ struct {
	Namespace   *string      `json:"namespace,omitempty"`
	Key         string       `json:"key"`
	Consistency *Consistency `json:"consistency,omitempty"`
}
