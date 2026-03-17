package coyote_models

// This file is @generated DO NOT EDIT

type KvGetIn struct {
	Consistency *Consistency `json:"consistency,omitempty"`
}

type KvGetIn_ struct {
	Key         string       `json:"key"`
	Consistency *Consistency `json:"consistency,omitempty"`
}
