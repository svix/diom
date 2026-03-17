package coyote_models

// This file is @generated DO NOT EDIT

type MsgNamespaceCreateIn struct {
	Retention   *Retention   `json:"retention,omitempty"`
	StorageType *StorageType `json:"storage_type,omitempty"`
}

type MsgNamespaceCreateIn_ struct {
	Name        string       `json:"name"`
	Retention   *Retention   `json:"retention,omitempty"`
	StorageType *StorageType `json:"storage_type,omitempty"`
}
