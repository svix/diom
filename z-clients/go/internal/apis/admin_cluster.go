package coyote_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	coyote_models "github.com/svix/coyote/z-clients/go/internal/models"
	coyote_proto "github.com/svix/coyote/z-clients/go/internal/proto"
)

type AdminCluster struct {
	client *coyote_proto.HttpClient
}

func NewAdminCluster(client *coyote_proto.HttpClient) AdminCluster {
	return AdminCluster{client}
}

// Get information about the current cluster
func (adminCluster AdminCluster) Status(
	ctx context.Context,
) (*coyote_models.ClusterStatusOut, error) {
	return coyote_proto.ExecuteRequest[any, coyote_models.ClusterStatusOut](
		ctx,
		adminCluster.client,
		"GET",
		"/api/v1/admin/cluster/status",
		nil,
	)
}
