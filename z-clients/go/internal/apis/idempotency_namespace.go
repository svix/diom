package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type IdempotencyNamespace struct {
	client *coyote_proto.HttpClient
}

func NewIdempotencyNamespace(client *coyote_proto.HttpClient) IdempotencyNamespace {
	return IdempotencyNamespace{client}
}

// Create idempotency namespace
func (idempotencyNamespace IdempotencyNamespace) Create(
	ctx context.Context,
	idempotencyCreateNamespaceIn coyote_models.IdempotencyCreateNamespaceIn,
) (*coyote_models.IdempotencyCreateNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyCreateNamespaceIn, coyote_models.IdempotencyCreateNamespaceOut](
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
	idempotencyGetNamespaceIn coyote_models.IdempotencyGetNamespaceIn,
) (*coyote_models.IdempotencyGetNamespaceOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyGetNamespaceIn, coyote_models.IdempotencyGetNamespaceOut](
		ctx,
		idempotencyNamespace.client,
		"POST",
		"/api/v1/idempotency/namespace/get",
		&idempotencyGetNamespaceIn,
	)
}
