package coyote_models

// This file is @generated DO NOT EDIT

type AdminAccessPolicyListIn struct {
	Limit    *uint64 `msgpack:"limit,omitempty"`    // Limit the number of returned items
	Iterator *string `msgpack:"iterator,omitempty"` // The iterator returned from a prior invocation
}
