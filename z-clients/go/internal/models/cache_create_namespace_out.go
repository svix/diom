package diom_models

// This file is @generated DO NOT EDIT

import (
	"time"
)

type CacheCreateNamespaceOut struct {
	Name           string         `msgpack:"name"`
	EvictionPolicy EvictionPolicy `msgpack:"eviction_policy"`
	Created        time.Time      `msgpack:"created"`
	Updated        time.Time      `msgpack:"updated"`
}
