package diom_models

// This file is @generated DO NOT EDIT

type CacheDeleteIn struct {
	Namespace *string `json:"namespace,omitempty"`
}

type CacheDeleteIn_ struct {
	Namespace *string `json:"namespace,omitempty"`
	Key       string  `json:"key"`
}
