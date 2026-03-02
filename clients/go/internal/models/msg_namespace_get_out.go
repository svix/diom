package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type MsgNamespaceGetOut struct {
	Name        string      `json:"name"`
	Retention   Retention   `json:"retention"`
	StorageType StorageType `json:"storage_type"`
	Created     time.Time   `json:"created"`
	Updated     time.Time   `json:"updated"`
}
