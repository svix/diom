package diom_models

// This file is @generated DO NOT EDIT

type SvixPollerOut struct {
	Topic    string `msgpack:"topic"`
	PollerId string `msgpack:"poller_id"`
	Token    string `msgpack:"token"` // The autoconfig token, obfuscated (e.g. `auto_v1_eyJh...fQ==`).
}
