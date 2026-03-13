package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type RateLimitNamespace struct {
	client *coyote_proto.HttpClient
}

func NewRateLimitNamespace(client *coyote_proto.HttpClient) RateLimitNamespace {
	return RateLimitNamespace{client}
}

// Create rate limiter namespace
func (rateLimitNamespace RateLimitNamespace) Create(
	ctx context.Context,
	rateLimitCreateNamespaceIn coyote_models.RateLimitCreateNamespaceIn,
) (*coyote_models.RateLimitCreateNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimitCreateNamespaceIn, coyote_models.RateLimitCreateNamespaceOut](
		ctx,
		rateLimitNamespace.client,
		"POST",
		"/api/v1/rate-limit/namespace/create",
		nil,
		nil,
		&rateLimitCreateNamespaceIn,
	)
}

// Get rate limiter namespace
func (rateLimitNamespace RateLimitNamespace) Get(
	ctx context.Context,
	rateLimitGetNamespaceIn coyote_models.RateLimitGetNamespaceIn,
) (*coyote_models.RateLimitGetNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.RateLimitGetNamespaceIn, coyote_models.RateLimitGetNamespaceOut](
		ctx,
		rateLimitNamespace.client,
		"POST",
		"/api/v1/rate-limit/namespace/get",
		nil,
		nil,
		&rateLimitGetNamespaceIn,
	)
}
