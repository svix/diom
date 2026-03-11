package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type RateLimiterNamespace struct {
	client *diom_proto.HttpClient
}

func NewRateLimiterNamespace(client *diom_proto.HttpClient) RateLimiterNamespace {
	return RateLimiterNamespace{client}
}

// Create rate limiter namespace
func (rateLimiterNamespace RateLimiterNamespace) Create(
	ctx context.Context,
	rateLimiterCreateNamespaceIn diom_models.RateLimiterCreateNamespaceIn,
) (*diom_models.RateLimiterCreateNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimiterCreateNamespaceIn, diom_models.RateLimiterCreateNamespaceOut](
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
	rateLimiterGetNamespaceIn diom_models.RateLimiterGetNamespaceIn,
) (*diom_models.RateLimiterGetNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimiterGetNamespaceIn, diom_models.RateLimiterGetNamespaceOut](
		ctx,
		rateLimiterNamespace.client,
		"POST",
		"/api/v1/rate-limit/namespace/get",
		nil,
		nil,
		&rateLimiterGetNamespaceIn,
	)
}
