package diom_models

// This file is @generated DO NOT EDIT

type SvixPollerDeleteIn struct {
	Namespace *string `msgpack:"namespace,omitempty"`
	Topic     string  `msgpack:"topic"`
	PollerId  string  `msgpack:"poller_id"`
}
