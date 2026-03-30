package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type Transformations struct {
	client *coyote_proto.HttpClient
}

func NewTransformations(client *coyote_proto.HttpClient) Transformations {
	return Transformations{client}
}

// Execute a JavaScript transformation script against a payload and return the result.
func (transformations Transformations) Execute(
	ctx context.Context,
	transformIn coyote_models.TransformIn,
) (*coyote_models.TransformOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.TransformIn, coyote_models.TransformOut](
		ctx,
		transformations.client,
		"POST",
		"/api/v1.transformations.execute",
		&transformIn,
	)
}
