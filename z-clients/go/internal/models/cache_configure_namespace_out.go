package diom_models

// This file is @generated DO NOT EDIT

type CacheConfigureNamespaceOut struct {
	Name           string         `msgpack:"name"`
	EvictionPolicy EvictionPolicy `msgpack:"eviction_policy"`
	Created        uint64         `msgpack:"created"`
	Updated        uint64         `msgpack:"updated"`
}
