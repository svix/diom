package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type Idempotency struct {
	client *coyote_proto.HttpClient
}

func NewIdempotency(client *coyote_proto.HttpClient) Idempotency {
	return Idempotency{client}
}

func (idempotency Idempotency) Namespace() IdempotencyNamespace {
	return NewIdempotencyNamespace(idempotency.client)
}

// Start an idempotent request
func (idempotency Idempotency) Start(
	ctx context.Context,
	key string,
	idempotencyStartIn coyote_models.IdempotencyStartIn,
) (*coyote_models.IdempotencyStartOut, error) {
	body := coyote_models.IdempotencyStartIn_{
		Namespace:    idempotencyStartIn.Namespace,
		Key:          key,
		LockPeriodMs: idempotencyStartIn.LockPeriodMs,
	}

	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyStartIn_, coyote_models.IdempotencyStartOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1.idempotency.start",
		&body,
	)
}

// Complete an idempotent request with a response
func (idempotency Idempotency) Complete(
	ctx context.Context,
	key string,
	idempotencyCompleteIn coyote_models.IdempotencyCompleteIn,
) (*coyote_models.IdempotencyCompleteOut, error) {
	body := coyote_models.IdempotencyCompleteIn_{
		Namespace: idempotencyCompleteIn.Namespace,
		Key:       key,
		Response:  idempotencyCompleteIn.Response,
		Context:   idempotencyCompleteIn.Context,
		TtlMs:     idempotencyCompleteIn.TtlMs,
	}

	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyCompleteIn_, coyote_models.IdempotencyCompleteOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1.idempotency.complete",
		&body,
	)
}

// Abandon an idempotent request (remove lock without saving response)
func (idempotency Idempotency) Abort(
	ctx context.Context,
	key string,
	idempotencyAbortIn coyote_models.IdempotencyAbortIn,
) (*coyote_models.IdempotencyAbortOut, error) {
	body := coyote_models.IdempotencyAbortIn_{
		Namespace: idempotencyAbortIn.Namespace,
		Key:       key,
	}

	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyAbortIn_, coyote_models.IdempotencyAbortOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1.idempotency.abort",
		&body,
	)
}
