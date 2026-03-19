package diom_models

// This file is @generated DO NOT EDIT

type KvGetIn struct {
	Namespace   *string      `json:"namespace,omitempty"`
	Consistency *Consistency `json:"consistency,omitempty"`
}

type KvGetIn_ struct {
	Namespace   *string      `json:"namespace,omitempty"`
	Key         string       `json:"key"`
	Consistency *Consistency `json:"consistency,omitempty"`
}
