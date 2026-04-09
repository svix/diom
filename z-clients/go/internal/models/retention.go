package coyote_models

// This file is @generated DO NOT EDIT

import (
	coyote_types "github.com/svix/coyote/z-clients/go/internal/types"
)

type Retention struct {
	Period *coyote_types.DurationMs `msgpack:"period_ms,omitempty"`
}
