package diom_models

// This file is @generated DO NOT EDIT

type SinkOut struct {
	Topic                   string        `msgpack:"topic"`
	ConsumerGroup           string        `msgpack:"consumer_group"`
	DefaultStartingPosition *SeekPosition `msgpack:"default_starting_position,omitempty"` // Where a freshly-created sink starts consuming the topic. Defaults to `earliest`.
	MaxInFlight             *uint32       `msgpack:"max_in_flight,omitempty"`             // At most how many concurrent requests will be sent to the Sink.
	Config                  SinkConfig    `msgpack:"config"`
}
