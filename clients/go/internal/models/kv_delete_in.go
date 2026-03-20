package coyote_models

// This file is @generated DO NOT EDIT

type KvDeleteIn struct {
	Namespace *string `json:"namespace,omitempty"`
}

type KvDeleteIn_ struct {
	Namespace *string `json:"namespace,omitempty"`
	Key       string  `json:"key"`
}
