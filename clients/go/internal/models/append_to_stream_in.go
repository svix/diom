package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type AppendToStreamIn struct {
	Msgs []MsgIn `json:"msgs"`
Name string `json:"name"`
}
