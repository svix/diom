package diom_models

// This file is @generated DO NOT EDIT

import (
	diom_types "github.com/svix/diom/z-clients/go/internal/types"
)

type Retention struct {
	Period *diom_types.DurationMs `msgpack:"period_ms,omitempty"`
}
