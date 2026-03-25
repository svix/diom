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

// Create rate limiter namespace
func (rateLimitNamespace RateLimitNamespace) Create(
	ctx context.Context,
	rateLimitCreateNamespaceIn diom_models.RateLimitCreateNamespaceIn,
) (*diom_models.RateLimitCreateNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.RateLimitCreateNamespaceIn, diom_models.RateLimitCreateNamespaceOut](
		ctx,
		rateLimitNamespace.client,
		"POST",
		"/api/v1.rate-limit.namespace.create",
		&rateLimitCreateNamespaceIn,
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
