package diom_models

// This file is @generated DO NOT EDIT

type SinkListIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Limit     *uint64 `msgpack:"limit,omitempty"`    // Limit the number of returned items
	Iterator  *string `msgpack:"iterator,omitempty"` // The iterator returned from a prior invocation
}

type SinkListIn_ struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Topic     string  `msgpack:"topic"`
	Limit     *uint64 `msgpack:"limit,omitempty"`    // Limit the number of returned items
	Iterator  *string `msgpack:"iterator,omitempty"` // The iterator returned from a prior invocation
}
