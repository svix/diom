package coyote_models

// This file is @generated DO NOT EDIT

import (
	"time"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type RateLimiterFixedWindowConfig struct {
	MaxRequests uint64 `json:"max_requests"`// Maximum number of requests allowed within the window
WindowSize uint64 `json:"window_size"`// Window size in seconds
}
