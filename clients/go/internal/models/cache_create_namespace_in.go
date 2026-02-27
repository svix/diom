package diom_models

// This file is @generated DO NOT EDIT

type CacheCreateNamespaceIn struct {
	Name            string          `json:"name"`
	StorageType     *StorageType    `json:"storage_type,omitempty"`
	MaxStorageBytes *uint64         `json:"max_storage_bytes,omitempty"`
	EvictionPolicy  *EvictionPolicy `json:"eviction_policy,omitempty"`
}
