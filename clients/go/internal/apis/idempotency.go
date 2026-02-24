package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Idempotency struct {
	client *coyote_proto.HttpClient
}

func NewIdempotency(client *coyote_proto.HttpClient) Idempotency {
	return Idempotency{client}
}

// Abandon an idempotent request (remove lock without saving response)
func (idempotency *Idempotency) Abort(
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
		nil,
		&idempotencyAbortIn,
	)
}

// Get idempotency group
func (idempotency *Idempotency) GetGroup(
	ctx context.Context,
	idempotencyGetGroupIn coyote_models.IdempotencyGetGroupIn,
) (*coyote_models.IdempotencyGetGroupOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyGetGroupIn, coyote_models.IdempotencyGetGroupOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/get-group",
		nil,
		nil,
		nil,
		&idempotencyGetGroupIn,
	)
}
