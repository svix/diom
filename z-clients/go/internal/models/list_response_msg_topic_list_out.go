package diom_models

// This file is @generated DO NOT EDIT

type ListResponseMsgTopicListOut struct {
	Data         []MsgTopicListOut `msgpack:"data"`
	Iterator     *string           `msgpack:"iterator,omitempty"`
	PrevIterator *string           `msgpack:"prev_iterator,omitempty"`
	Done         bool              `msgpack:"done"`
}
