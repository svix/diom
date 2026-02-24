package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Idempotency struct {
	client *diom_proto.HttpClient
}

func NewIdempotency(client *diom_proto.HttpClient) Idempotency {
	return Idempotency{client}
}

// Abandon an idempotent request (remove lock without saving response)
func (idempotency *Idempotency) Abort(
	ctx context.Context,
	idempotencyAbortIn diom_models.IdempotencyAbortIn,
) (*diom_models.IdempotencyAbortOut, error) {
	return diom_proto.ExecuteRequest[diom_models.IdempotencyAbortIn, diom_models.IdempotencyAbortOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/abort",
		nil,
		nil,
		&idempotencyAbortIn,
	)
}

// Get idempotency group
func (idempotency *Idempotency) GetGroup(
	ctx context.Context,
	idempotencyGetGroupIn diom_models.IdempotencyGetGroupIn,
) (*diom_models.IdempotencyGetGroupOut, error) {
	return diom_proto.ExecuteRequest[diom_models.IdempotencyGetGroupIn, diom_models.IdempotencyGetGroupOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/get-group",
		nil,
		nil,
		&idempotencyGetGroupIn,
	)
}
