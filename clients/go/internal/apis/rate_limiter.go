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

type RateLimiterLimitOptions struct {
	IdempotencyKey *string
}

type RateLimiterGetRemainingOptions struct {
	IdempotencyKey *string
}

// Rate Limiter Check and Consume
func (rateLimiter *RateLimiter) Limit(
	ctx context.Context,
	rateLimiterCheckIn diom_models.RateLimiterCheckIn,
	o *RateLimiterLimitOptions,
) (*diom_models.RateLimiterCheckOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.RateLimiterCheckIn, diom_models.RateLimiterCheckOut](
		ctx,
		rateLimiter.client,
		"POST",
		"/api/v1/rate-limiter/limit",
		nil,
		nil,
		headerMap,
		&rateLimiterCheckIn,
	)
}

// Rate Limiter Get Remaining
func (rateLimiter *RateLimiter) GetRemaining(
	ctx context.Context,
	rateLimiterGetRemainingIn diom_models.RateLimiterGetRemainingIn,
	o *RateLimiterGetRemainingOptions,
) (*diom_models.RateLimiterGetRemainingOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.RateLimiterGetRemainingIn, diom_models.RateLimiterGetRemainingOut](
		ctx,
		rateLimiter.client,
		"POST",
		"/api/v1/rate-limiter/get-remaining",
		nil,
		nil,
		headerMap,
		&rateLimiterGetRemainingIn,
	)
}
