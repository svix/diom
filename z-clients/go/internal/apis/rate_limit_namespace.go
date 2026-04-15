package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type RateLimitNamespace struct {
	client *diom_proto.HttpClient
}

func NewRateLimitNamespace(client *diom_proto.HttpClient) RateLimitNamespace {
	return RateLimitNamespace{client}
}

// Configure rate limiter namespace
func (rateLimitNamespace RateLimitNamespace) Configure(
	ctx context.Context,
	rateLimitConfigureNamespaceIn diom_models.RateLimitConfigureNamespaceIn,
) (*diom_models.RateLimitConfigureNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimitConfigureNamespaceIn, diom_models.RateLimitConfigureNamespaceOut](
		ctx,
		rateLimitNamespace.client,
		"POST",
		"/api/v1.rate-limit.namespace.configure",
		&rateLimitConfigureNamespaceIn,
	)
}

// Get rate limiter namespace
func (rateLimitNamespace RateLimitNamespace) Get(
	ctx context.Context,
	rateLimitGetNamespaceIn diom_models.RateLimitGetNamespaceIn,
) (*diom_models.RateLimitGetNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimitGetNamespaceIn, diom_models.RateLimitGetNamespaceOut](
		ctx,
		rateLimitNamespace.client,
		"POST",
		"/api/v1.rate-limit.namespace.get",
		&rateLimitGetNamespaceIn,
	)
}
