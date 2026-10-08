package diom_models

// This file is @generated DO NOT EDIT

type MsgTopicDescribeOut struct {
	// The unique internal ID of this topic
	//
	// This can useful for debugging
	Id         string                         `msgpack:"id"`
	Name       string                         `msgpack:"name"`
	Partitions []MsgTopicPartitionDescribeOut `msgpack:"partitions"`
}
