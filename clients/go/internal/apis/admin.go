package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/clients/go/internal/proto"
)

type Admin struct {
	client *coyote_proto.HttpClient
}

func NewAdmin(client *coyote_proto.HttpClient) Admin {
	return Admin{client}
}

// Get information about the current cluster
func (admin Admin) ClusterStatus(
	ctx context.Context,
) (*coyote_models.ClusterStatusOut, error) {
	return coyote_proto.ExecuteRequest[any, coyote_models.ClusterStatusOut](
		ctx,
		admin.client,
		"GET",
		"/api/v1/admin/cluster-status",
		nil,
		nil,
		nil,
	)
}
