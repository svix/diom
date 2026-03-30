package diom_models

// This file is @generated DO NOT EDIT

type CacheCreateNamespaceIn struct {
	Name           string          `msgpack:"name"`
	EvictionPolicy *EvictionPolicy `msgpack:"eviction_policy,omitempty"`
}
