package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type RateLimiterGetRemainingOut struct {
	Remaining uint64 `json:"remaining"`// Number of tokens remaining
RetryAfter *uint64 `json:"retry_after,omitempty"`// Seconds until at least one token is available (only present when remaining is 0)
}
