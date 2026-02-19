package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type AppendToStreamOut struct {
	MsgIds []uint64 `json:"msg_ids"`
}
