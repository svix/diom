package diom_models

// This file is @generated DO NOT EDIT

type SvixPollerDeleteOut struct {
	Topic    string `msgpack:"topic"`
	PollerId string `msgpack:"poller_id"`
	Success  bool   `msgpack:"success"`
}
