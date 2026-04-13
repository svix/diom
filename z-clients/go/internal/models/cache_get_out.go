package diom_models

// This file is @generated DO NOT EDIT

type CacheGetOut struct {
	Expiry *uint64 `msgpack:"expiry,omitempty"` // Time of expiry
	Value  []uint8 `msgpack:"value,omitempty"`
}
