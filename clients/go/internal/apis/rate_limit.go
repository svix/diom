package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type RateLimit struct {
	client *coyote_proto.HttpClient
}

func NewRateLimit(client *coyote_proto.HttpClient) RateLimit {
	return RateLimit{client}
}

func (rateLimit RateLimit) Namespace() RateLimitNamespace {
	return NewRateLimitNamespace(rateLimit.client)
}

// Rate Limiter Check and Consume
func (rateLimit RateLimit) Limit(
	ctx context.Context,
	rateLimitCheckIn coyote_models.RateLimitCheckIn,
) (*coyote_models.RateLimitCheckOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimitCheckIn, coyote_models.RateLimitCheckOut](
		ctx,
		rateLimit.client,
		"POST",
		"/api/v1/rate-limit/limit",
		&rateLimitCheckIn,
	)
}

// Rate Limiter Get Remaining
func (rateLimit RateLimit) GetRemaining(
	ctx context.Context,
	rateLimitGetRemainingIn coyote_models.RateLimitGetRemainingIn,
) (*coyote_models.RateLimitGetRemainingOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimitGetRemainingIn, coyote_models.RateLimitGetRemainingOut](
		ctx,
		rateLimit.client,
		"POST",
		"/api/v1/rate-limit/get-remaining",
		&rateLimitGetRemainingIn,
	)
}

// Rate Limiter Reset
func (rateLimit RateLimit) Reset(
	ctx context.Context,
	rateLimitResetIn coyote_models.RateLimitResetIn,
) (*coyote_models.RateLimitResetOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimitResetIn, coyote_models.RateLimitResetOut](
		ctx,
		rateLimit.client,
		"POST",
		"/api/v1/rate-limit/reset",
		&rateLimitResetIn,
	)
}
