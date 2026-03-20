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

func (admin Admin) Cluster() AdminCluster {
	return NewAdminCluster(admin.client)
}

// Remove a node from the cluster.
//
// This operation executes immediately and the node must be wiped and reset
// before it can safely be added to the cluster.
func (admin Admin) ClusterRemoveNode(
	ctx context.Context,
	clusterRemoveNodeIn coyote_models.ClusterRemoveNodeIn,
) (*coyote_models.ClusterRemoveNodeOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.ClusterRemoveNodeIn, coyote_models.ClusterRemoveNodeOut](
		ctx,
		admin.client,
		"POST",
		"/api/v1/admin/cluster/remove-node",
		&clusterRemoveNodeIn,
	)
}
