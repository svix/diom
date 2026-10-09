package diom_models

// This file is @generated DO NOT EDIT

type MsgTopicPartitionDescribeOut struct {
	PartitionId   uint16 `msgpack:"partition_id"`
	HighWaterMark uint64 `msgpack:"high_water_mark"` // The next offset to be committed to this partition
}
