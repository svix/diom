package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type RateLimiterNamespace struct {
	client *coyote_proto.HttpClient
}

func NewRateLimiterNamespace(client *coyote_proto.HttpClient) RateLimiterNamespace {
	return RateLimiterNamespace{client}
}

// Create rate limiter namespace
func (rateLimiterNamespace RateLimiterNamespace) Create(
	ctx context.Context,
	rateLimiterCreateNamespaceIn coyote_models.RateLimiterCreateNamespaceIn,
) (*coyote_models.RateLimiterCreateNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimiterCreateNamespaceIn, coyote_models.RateLimiterCreateNamespaceOut](
		ctx,
		rateLimiterNamespace.client,
		"POST",
		"/api/v1/rate-limit/namespace/create",
		nil,
		nil,
		&rateLimiterCreateNamespaceIn,
	)
}

// Get rate limiter namespace
func (rateLimiterNamespace RateLimiterNamespace) Get(
	ctx context.Context,
	rateLimiterGetNamespaceIn coyote_models.RateLimiterGetNamespaceIn,
) (*coyote_models.RateLimiterGetNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimiterGetNamespaceIn, coyote_models.RateLimiterGetNamespaceOut](
		ctx,
		rateLimiterNamespace.client,
		"POST",
		"/api/v1/rate-limit/namespace/get",
		nil,
		nil,
		&rateLimiterGetNamespaceIn,
	)
}
