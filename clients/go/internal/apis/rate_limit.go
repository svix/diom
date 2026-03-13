package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type RateLimit struct {
	client    *diom_proto.HttpClient
	Namespace *RateLimitNamespace
}

func NewRateLimit(client *diom_proto.HttpClient) RateLimit {
	return RateLimit{client}
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
		"/api/v1/rate-limit/limit",
		nil,
		nil,
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
		"/api/v1/rate-limit/get-remaining",
		nil,
		nil,
		&rateLimitGetRemainingIn,
	)
}
