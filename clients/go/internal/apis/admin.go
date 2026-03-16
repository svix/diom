package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type Admin struct {
	client *diom_proto.HttpClient
}

func NewAdmin(client *diom_proto.HttpClient) Admin {
	return Admin{client}
}

// Get information about the current cluster
func (admin Admin) ClusterStatus(
	ctx context.Context,
) (*diom_models.ClusterStatusOut, error) {
	return diom_proto.ExecuteRequest[any, diom_models.ClusterStatusOut](
		ctx,
		admin.client,
		"GET",
		"/api/v1/admin/cluster-status",
		nil,
		nil,
		nil,
	)
}
