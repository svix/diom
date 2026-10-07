package diom_models

// This file is @generated DO NOT EDIT

type MsgTopicListIn struct {
	Namespace   *string      `msgpack:"namespace,omitempty"`
	Consistency *Consistency `msgpack:"consistency,omitempty"`
	Limit       *uint64      `msgpack:"limit,omitempty"`    // Limit the number of returned items
	Iterator    *string      `msgpack:"iterator,omitempty"` // The iterator returned from a prior invocation
}
