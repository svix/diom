package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type RateLimiter struct {
	client *diom_proto.HttpClient
}

func NewRateLimiter(client *diom_proto.HttpClient) RateLimiter {
	return RateLimiter{client}
}

// Rate Limiter Check and Consume
func (rateLimiter *RateLimiter) Limit(
	ctx context.Context,
	rateLimiterCheckIn diom_models.RateLimiterCheckIn,
) (*diom_models.RateLimiterCheckOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimiterCheckIn, diom_models.RateLimiterCheckOut](
		ctx,
		rateLimiter.client,
		"POST",
		"/api/v1/rate-limiter/limit",
		nil,
		nil,
		nil,
		&rateLimiterCheckIn,
	)
}

// Rate Limiter Get Remaining
func (rateLimiter *RateLimiter) GetRemaining(
	ctx context.Context,
	rateLimiterGetRemainingIn diom_models.RateLimiterGetRemainingIn,
) (*diom_models.RateLimiterGetRemainingOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimiterGetRemainingIn, diom_models.RateLimiterGetRemainingOut](
		ctx,
		rateLimiter.client,
		"POST",
		"/api/v1/rate-limiter/get-remaining",
		nil,
		nil,
		nil,
		&rateLimiterGetRemainingIn,
	)
}
