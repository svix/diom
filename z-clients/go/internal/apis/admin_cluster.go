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
		"/api/v1.admin.cluster.status",
		nil,
	)
}

// Initialize this node as the leader of a new cluster
//
// This operation may only be performed against a node which has not been
// initialized and is not currently a member of a cluster.
func (adminCluster AdminCluster) Initialize(
	ctx context.Context,
	clusterInitializeIn coyote_models.ClusterInitializeIn,
) (*coyote_models.ClusterInitializeOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.ClusterInitializeIn, coyote_models.ClusterInitializeOut](
		ctx,
		adminCluster.client,
		"POST",
		"/api/v1.admin.cluster.initialize",
		&clusterInitializeIn,
	)
}

// Remove a node from the cluster.
//
// This operation executes immediately and the node must be wiped and reset
// before it can safely be added to the cluster.
func (adminCluster AdminCluster) RemoveNode(
	ctx context.Context,
	clusterRemoveNodeIn coyote_models.ClusterRemoveNodeIn,
) (*coyote_models.ClusterRemoveNodeOut, error) {
	return coyote_proto.ExecuteRequest[coyote_models.ClusterRemoveNodeIn, coyote_models.ClusterRemoveNodeOut](
		ctx,
		adminCluster.client,
		"POST",
		"/api/v1.admin.cluster.remove-node",
		&clusterRemoveNodeIn,
	)
}
