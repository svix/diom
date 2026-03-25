package coyote_models

// This file is @generated DO NOT EDIT

type TransformIn struct {
	Input         string  `msgpack:"input"`                     // JSON-encoded payload passed to the script as `input`.
	Script        string  `msgpack:"script"`                    // JavaScript source. Must define a `handler(input)` function.
	MaxDurationMs *uint64 `msgpack:"max_duration_ms,omitempty"` // How long to let the script run before being killed.
}
