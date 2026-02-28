package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type MsgNamespaceGetOut struct {
	Created     time.Time   `json:"created"`
	Name        string      `json:"name"`
	Retention   Retention   `json:"retention"`
	StorageType StorageType `json:"storage_type"`
	Updated     time.Time   `json:"updated"`
}
