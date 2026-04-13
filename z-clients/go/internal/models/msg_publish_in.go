package diom_models

// This file is @generated DO NOT EDIT

type MsgPublishIn struct {
	Namespace      *string `msgpack:"namespace,omitempty"`
	Msgs           []MsgIn `msgpack:"msgs"`
	IdempotencyKey *string `msgpack:"idempotency_key,omitempty"`
}

type MsgPublishIn_ struct {
	Namespace      *string `msgpack:"namespace,omitempty"`
	Topic          string  `msgpack:"topic"`
	Msgs           []MsgIn `msgpack:"msgs"`
	IdempotencyKey *string `msgpack:"idempotency_key,omitempty"`
}
