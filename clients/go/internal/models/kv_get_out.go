package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type KvGetOut struct {
	Expiry *time.Time `json:"expiry,omitempty"`// Time of expiry
Key string `json:"key"`
Value []uint8 `json:"value"`
}
