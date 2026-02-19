package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type CacheSetIn struct {
	Key string `json:"key"`
Ttl uint64 `json:"ttl"`// Time to live in milliseconds
Value []uint8 `json:"value"`
}
