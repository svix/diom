package coyote_models

// This file is @generated DO NOT EDIT

type ListResponseAuthTokenOut struct {
	Data         []AuthTokenOut `msgpack:"data"`
	Iterator     *string        `msgpack:"iterator,omitempty"`
	PrevIterator *string        `msgpack:"prev_iterator,omitempty"`
	Done         bool           `msgpack:"done"`
}
