package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/clients/go/internal/models"
	diom_proto "github.com/svix/diom/clients/go/internal/proto"
)

type AdminCluster struct {
	client *diom_proto.HttpClient
}

func NewAdminCluster(client *diom_proto.HttpClient) AdminCluster {
	return AdminCluster{client}
}

// Get information about the current cluster
func (adminCluster AdminCluster) Status(
	ctx context.Context,
) (*diom_models.ClusterStatusOut, error) {
	return diom_proto.ExecuteRequest[any, diom_models.ClusterStatusOut](
		ctx,
		adminCluster.client,
		"GET",
		"/api/v1/admin/cluster/status",
		nil,
	)
}
