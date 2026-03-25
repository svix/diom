package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenCreateNamespaceIn struct {
	Name            string  `msgpack:"name"`
	MaxStorageBytes *uint64 `msgpack:"max_storage_bytes,omitempty"`
}
