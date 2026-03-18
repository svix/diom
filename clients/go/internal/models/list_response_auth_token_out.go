package coyote_models

// This file is @generated DO NOT EDIT

type ListResponseAuthTokenOut struct {
	Data         []AuthTokenOut `json:"data"`
	Iterator     *string        `json:"iterator,omitempty"`
	PrevIterator *string        `json:"prev_iterator,omitempty"`
	Done         bool           `json:"done"`
}
