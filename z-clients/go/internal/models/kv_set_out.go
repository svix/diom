package diom_models

// This file is @generated DO NOT EDIT

type KvSetOut struct {
	Success bool   `msgpack:"success"` // Whether the operation succeeded or was a noop due to pre-conditions.
	Version uint64 `msgpack:"version"`
}
