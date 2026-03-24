package diom_models

// This file is @generated DO NOT EDIT

type CacheCreateNamespaceIn struct {
	Name            string          `msgpack:"name"`
	StorageType     *StorageType    `msgpack:"storage_type,omitempty"`
	MaxStorageBytes *uint64         `msgpack:"max_storage_bytes,omitempty"`
	EvictionPolicy  *EvictionPolicy `msgpack:"eviction_policy,omitempty"`
}
