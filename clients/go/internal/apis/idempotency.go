package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Idempotency struct {
	client    *coyote_proto.HttpClient
	Namespace *IdempotencyNamespace
}

func NewIdempotency(client *coyote_proto.HttpClient) Idempotency {
	return Idempotency{client}
}

// Start an idempotent request
func (idempotency Idempotency) Start(
	ctx context.Context,
	idempotencyStartIn coyote_models.IdempotencyStartIn,
) (*coyote_models.IdempotencyStartOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyStartIn, coyote_models.IdempotencyStartOut](
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
	idempotencyCompleteIn coyote_models.IdempotencyCompleteIn,
) (*coyote_models.IdempotencyCompleteOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyCompleteIn, coyote_models.IdempotencyCompleteOut](
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
	idempotencyAbortIn coyote_models.IdempotencyAbortIn,
) (*coyote_models.IdempotencyAbortOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyAbortIn, coyote_models.IdempotencyAbortOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/abort",
		nil,
		nil,
		&idempotencyAbortIn,
	)
}
