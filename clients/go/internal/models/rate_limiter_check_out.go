package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type RateLimiterCheckOut struct {
	Remaining uint64 `json:"remaining"`// Number of tokens remaining
RetryAfter *uint64 `json:"retry_after,omitempty"`// Seconds until enough tokens are available (only present when allowed is false)
Status RateLimitStatus `json:"status"`// Whether the request is allowed
}
