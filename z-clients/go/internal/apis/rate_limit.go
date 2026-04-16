package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
)

type RateLimit struct {
	client *diom_proto.HttpClient
}

func NewRateLimit(client *diom_proto.HttpClient) RateLimit {
	return RateLimit{client}
}

func (rateLimit RateLimit) Namespace() RateLimitNamespace {
	return NewRateLimitNamespace(rateLimit.client)
}

// Rate Limiter Check and Consume
func (rateLimit RateLimit) Limit(
	ctx context.Context,
	rateLimitCheckIn diom_models.RateLimitCheckIn,
) (*diom_models.RateLimitCheckOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimitCheckIn, diom_models.RateLimitCheckOut](
		ctx,
		rateLimit.client,
		"POST",
		"/api/v1.rate-limit.limit",
		&rateLimitCheckIn,
	)
}

// Rate Limiter Get Remaining
func (rateLimit RateLimit) GetRemaining(
	ctx context.Context,
	rateLimitGetRemainingIn diom_models.RateLimitGetRemainingIn,
) (*diom_models.RateLimitGetRemainingOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimitGetRemainingIn, diom_models.RateLimitGetRemainingOut](
		ctx,
		rateLimit.client,
		"POST",
		"/api/v1.rate-limit.get-remaining",
		&rateLimitGetRemainingIn,
	)
}

// Rate Limiter Reset
func (rateLimit RateLimit) Reset(
	ctx context.Context,
	rateLimitResetIn diom_models.RateLimitResetIn,
) (*diom_models.RateLimitResetOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimitResetIn, diom_models.RateLimitResetOut](
		ctx,
		rateLimit.client,
		"POST",
		"/api/v1.rate-limit.reset",
		&rateLimitResetIn,
	)
}
