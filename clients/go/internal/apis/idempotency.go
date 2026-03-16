package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Idempotency struct {
	client    *diom_proto.HttpClient
	Namespace *IdempotencyNamespace
}

func NewIdempotency(client *diom_proto.HttpClient) Idempotency {
	return Idempotency{client}
}

// Start an idempotent request
func (idempotency Idempotency) Start(
	ctx context.Context,
	idempotencyStartIn diom_models.IdempotencyStartIn,
) (*diom_models.IdempotencyStartOut, error) {
	return diom_proto.ExecuteRequest[diom_models.IdempotencyStartIn, diom_models.IdempotencyStartOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/start",
		nil,
		nil,
		&idempotencyStartIn,
	)
}

// Complete an idempotent request with a response
func (idempotency Idempotency) Complete(
	ctx context.Context,
	idempotencyCompleteIn diom_models.IdempotencyCompleteIn,
) (*diom_models.IdempotencyCompleteOut, error) {
	return diom_proto.ExecuteRequest[diom_models.IdempotencyCompleteIn, diom_models.IdempotencyCompleteOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/complete",
		nil,
		nil,
		&idempotencyCompleteIn,
	)
}

// Abandon an idempotent request (remove lock without saving response)
func (idempotency Idempotency) Abort(
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
