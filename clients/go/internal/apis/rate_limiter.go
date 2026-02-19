package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
	coyote_models "github.com/svix/coyote/clients/go/internal/models"
)


type RateLimiter struct {
	client *coyote_proto.HttpClient
}

func NewRateLimiter(client *coyote_proto.HttpClient) RateLimiter {
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
	rateLimiterCheckIn coyote_models.RateLimiterCheckIn,
	o *RateLimiterLimitOptions,
	) (*coyote_models.RateLimiterCheckOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
			if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.RateLimiterCheckIn,coyote_models.RateLimiterCheckOut](
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
	rateLimiterGetRemainingIn coyote_models.RateLimiterGetRemainingIn,
	o *RateLimiterGetRemainingOptions,
	) (*coyote_models.RateLimiterGetRemainingOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
			if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.RateLimiterGetRemainingIn,coyote_models.RateLimiterGetRemainingOut](
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
	
