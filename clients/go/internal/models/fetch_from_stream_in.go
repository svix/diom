package diom_models

// This file is @generated DO NOT EDIT

type FetchFromStreamIn struct {
	BatchSize                uint16 `json:"batch_size"` // How many messages to read from the stream.
	ConsumerGroup            string `json:"consumer_group"`
	Name                     string `json:"name"`
	VisibilityTimeoutSeconds uint64 `json:"visibility_timeout_seconds"` // How long messages are locked by the consumer.
}
