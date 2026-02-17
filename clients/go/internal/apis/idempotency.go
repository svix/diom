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

type IdempotencyAbortOptions struct {
	IdempotencyKey *string
}

// Abandon an idempotent request (remove lock without saving response)
func (idempotency *Idempotency) Abort(
	ctx context.Context,
	idempotencyAbortIn diom_models.IdempotencyAbortIn,
	o *IdempotencyAbortOptions,
) (*diom_models.IdempotencyAbortOut, error) {
	headerMap := map[string]string{}
	var err error
	if o != nil {
		diom_proto.SerializeParamToMap("idempotency-key", o.IdempotencyKey, headerMap, &err)
		if err != nil {
			return nil, err
		}
	}
	return diom_proto.ExecuteRequest[diom_models.IdempotencyAbortIn, diom_models.IdempotencyAbortOut](
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
