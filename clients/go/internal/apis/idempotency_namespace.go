package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type IdempotencyNamespace struct {
	client *diom_proto.HttpClient
}

func NewIdempotencyNamespace(client *diom_proto.HttpClient) IdempotencyNamespace {
	return IdempotencyNamespace{client}
}

// Create idempotency namespace
func (idempotencyNamespace IdempotencyNamespace) Create(
	ctx context.Context,
	idempotencyCreateNamespaceIn diom_models.IdempotencyCreateNamespaceIn,
) (*diom_models.IdempotencyCreateNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.IdempotencyCreateNamespaceIn, diom_models.IdempotencyCreateNamespaceOut](
		ctx,
		idempotencyNamespace.client,
		"POST",
		"/api/v1/idempotency/namespace/create",
		&idempotencyCreateNamespaceIn,
	)
}

// Get idempotency namespace
func (idempotencyNamespace IdempotencyNamespace) Get(
	ctx context.Context,
	idempotencyGetNamespaceIn diom_models.IdempotencyGetNamespaceIn,
) (*diom_models.IdempotencyGetNamespaceOut, error) {
	return diom_proto.ExecuteRequest[diom_models.IdempotencyGetNamespaceIn, diom_models.IdempotencyGetNamespaceOut](
		ctx,
		idempotencyNamespace.client,
		"POST",
		"/api/v1/idempotency/namespace/get",
		&idempotencyGetNamespaceIn,
	)
}
