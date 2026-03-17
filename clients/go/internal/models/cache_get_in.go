package coyote_models

// This file is @generated DO NOT EDIT

type CacheGetIn struct {
	Consistency *Consistency `json:"consistency,omitempty"`
}

type CacheGetIn_ struct {
	Key         string       `json:"key"`
	Consistency *Consistency `json:"consistency,omitempty"`
}
