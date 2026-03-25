package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "github.com/svix/diom/z-clients/go/internal/models"
	diom_proto "github.com/svix/diom/z-clients/go/internal/proto"
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
	clusterInitializeIn diom_models.ClusterInitializeIn,
) (*diom_models.ClusterInitializeOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterInitializeIn, diom_models.ClusterInitializeOut](
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
	clusterRemoveNodeIn diom_models.ClusterRemoveNodeIn,
) (*diom_models.ClusterRemoveNodeOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterRemoveNodeIn, diom_models.ClusterRemoveNodeOut](
		ctx,
		adminCluster.client,
		"POST",
		"/api/v1.admin.cluster.remove-node",
		&clusterRemoveNodeIn,
	)
}
