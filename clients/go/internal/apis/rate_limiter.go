package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type RateLimiter struct {
	client *coyote_proto.HttpClient
}

func NewRateLimiter(client *coyote_proto.HttpClient) RateLimiter {
	return RateLimiter{client}
}

// Rate Limiter Check and Consume
func (rateLimiter RateLimiter) Limit(
	ctx context.Context,
	rateLimiterCheckIn coyote_models.RateLimiterCheckIn,
) (*coyote_models.RateLimiterCheckOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimiterCheckIn, coyote_models.RateLimiterCheckOut](
		ctx,
		rateLimiter.client,
		"POST",
		"/api/v1/rate-limiter/limit",
		nil,
		nil,
		&rateLimiterCheckIn,
	)
}

// Rate Limiter Get Remaining
func (rateLimiter RateLimiter) GetRemaining(
	ctx context.Context,
	rateLimiterGetRemainingIn coyote_models.RateLimiterGetRemainingIn,
) (*coyote_models.RateLimiterGetRemainingOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimiterGetRemainingIn, coyote_models.RateLimiterGetRemainingOut](
		ctx,
		rateLimiter.client,
		"POST",
		"/api/v1/rate-limiter/get-remaining",
		nil,
		nil,
		&rateLimiterGetRemainingIn,
	)
}
