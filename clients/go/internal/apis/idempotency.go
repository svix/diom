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

type IdempotencyAbortOptions struct {
	IdempotencyKey *string
}

// Abandon an idempotent request (remove lock without saving response)
func (idempotency *Idempotency) Abort(
	ctx context.Context,
	idempotencyAbortIn coyote_models.IdempotencyAbortIn,
	o *IdempotencyAbortOptions,
) (*coyote_models.IdempotencyAbortOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		coyote_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return coyote_proto.ExecuteRequest[coyote_models.IdempotencyAbortIn, coyote_models.IdempotencyAbortOut](
		ctx,
		idempotency.client,
		"POST",
		"/api/v1/idempotency/abort",
		nil,
		nil,
		headerMap,
		&idempotencyAbortIn,
	)
}
