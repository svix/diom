package diom_models

// This file is @generated DO NOT EDIT

type ListResponseAdminRoleOut struct {
	Data         []AdminRoleOut `msgpack:"data"`
	Iterator     *string        `msgpack:"iterator,omitempty"`
	PrevIterator *string        `msgpack:"prev_iterator,omitempty"`
	Done         bool           `msgpack:"done"`
}
