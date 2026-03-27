package diom_models

// This file is @generated DO NOT EDIT

type AuthTokenListIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	OwnerId   string  `msgpack:"owner_id"`
	Limit     *uint64 `msgpack:"limit,omitempty"`    // Limit the number of returned items
	Iterator  *string `msgpack:"iterator,omitempty"` // The iterator returned from a prior invocation
}
