package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type IdempotencyAbortIn struct {
	Key string `json:"key"`
}
