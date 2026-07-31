package diom_models

// This file is @generated DO NOT EDIT

type SinkConfigureIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	// The topic whose messages are forwarded to the sink. Created automatically if it does not
	// exist.
	Topic                   string        `msgpack:"topic"`
	ConsumerGroup           string        `msgpack:"consumer_group"`                      // The consumer group that identifies the sink and tracks its progress through the topic.
	DefaultStartingPosition *SeekPosition `msgpack:"default_starting_position,omitempty"` // Where a freshly-created sink starts consuming the topic. Defaults to `earliest`.
	MaxInFlight             *uint32       `msgpack:"max_in_flight,omitempty"`             // At most how many concurrent requests will be sent to the Sink.
	Config                  SinkConfig    `msgpack:"config"`
}
