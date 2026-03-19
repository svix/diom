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

func (idempotency Idempotency) Namespace() IdempotencyNamespace {
	return NewIdempotencyNamespace(idempotency.client)
}

// Start an idempotent request
func (idempotency Idempotency) Start(
	ctx context.Context,
	key string,
	idempotencyStartIn diom_models.IdempotencyStartIn,
) (*diom_models.IdempotencyStartOut, error) {
	body := diom_models.IdempotencyStartIn_{
		Namespace: idempotencyStartIn.Namespace,
		Key:       key,
		Ttl:       idempotencyStartIn.Ttl,
	}

	return diom_proto.ExecuteRequest[diom_models.IdempotencyStartIn_, diom_models.IdempotencyStartOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/start",
		&body,
	)
}

// Complete an idempotent request with a response
func (idempotency Idempotency) Complete(
	ctx context.Context,
	key string,
	idempotencyCompleteIn diom_models.IdempotencyCompleteIn,
) (*diom_models.IdempotencyCompleteOut, error) {
	body := diom_models.IdempotencyCompleteIn_{
		Namespace: idempotencyCompleteIn.Namespace,
		Key:       key,
		Response:  idempotencyCompleteIn.Response,
		Ttl:       idempotencyCompleteIn.Ttl,
	}

	return diom_proto.ExecuteRequest[diom_models.IdempotencyCompleteIn_, diom_models.IdempotencyCompleteOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/complete",
		&body,
	)
}

// Abandon an idempotent request (remove lock without saving response)
func (idempotency Idempotency) Abort(
	ctx context.Context,
	key string,
	idempotencyAbortIn diom_models.IdempotencyAbortIn,
) (*diom_models.IdempotencyAbortOut, error) {
	body := diom_models.IdempotencyAbortIn_{
		Namespace: idempotencyAbortIn.Namespace,
		Key:       key,
	}

	return diom_proto.ExecuteRequest[diom_models.IdempotencyAbortIn_, diom_models.IdempotencyAbortOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/abort",
		&body,
	)
}
