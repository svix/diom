package diom_models

// This file is @generated DO NOT EDIT

type IdempotencyCompleted struct {
	Response []uint8            `msgpack:"response"`
	Context  *map[string]string `msgpack:"context,omitempty"`
}
