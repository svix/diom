package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
)

type Transformations struct {
	client *diom_proto.HttpClient
}

func NewTransformations(client *diom_proto.HttpClient) Transformations {
	return Transformations{client}
}

// Execute a JavaScript transformation script against a payload and return the result.
func (transformations Transformations) Execute(
	ctx context.Context,
	transformIn diom_models.TransformIn,
) (*diom_models.TransformOut, error) {
	return diom_proto.ExecuteRequest[diom_models.TransformIn, diom_models.TransformOut](
		ctx,
		transformations.client,
		"POST",
		"/api/v1.transformations.execute",
		&transformIn,
	)
}
