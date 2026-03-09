package coyote_models

// This file is @generated DO NOT EDIT

type CacheGetIn struct {
	Key string `json:"key"`
	// Whether or not the read should be linearizable
	//
	// If this is `true`, the read is guaranteed to see all previous operations, but will
	// have to make at least one additional round-trip to the leader. If this is false, stale
	// reads will be performed against the replica which receives this request.
	Linearizable *bool `json:"linearizable,omitempty"`
}
