package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.svix.com/go/diom/internal/models"
	diom_proto "diom.svix.com/go/diom/internal/proto"
)

type ClusterAdmin struct {
	client *diom_proto.HttpClient
}

func NewClusterAdmin(client *diom_proto.HttpClient) ClusterAdmin {
	return ClusterAdmin{client}
}

// Get information about the current cluster
func (clusterAdmin ClusterAdmin) Status(
	ctx context.Context,
) (*diom_models.ClusterStatusOut, error) {
	return diom_proto.ExecuteRequest[any, diom_models.ClusterStatusOut](
		ctx,
		clusterAdmin.client,
		"GET",
		"/api/v1.cluster-admin.status",
		nil,
	)
}

// Initialize this node as the leader of a new cluster
//
// This operation may only be performed against a node which has not been
// initialized and is not currently a member of a cluster.
func (clusterAdmin ClusterAdmin) Initialize(
	ctx context.Context,
	clusterInitializeIn diom_models.ClusterInitializeIn,
) (*diom_models.ClusterInitializeOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterInitializeIn, diom_models.ClusterInitializeOut](
		ctx,
		clusterAdmin.client,
		"POST",
		"/api/v1.cluster-admin.initialize",
		&clusterInitializeIn,
	)
}

// Remove a node from the cluster.
//
// This operation executes immediately and the node must be wiped and reset
// before it can safely be added to the cluster.
func (clusterAdmin ClusterAdmin) RemoveNode(
	ctx context.Context,
	clusterRemoveNodeIn diom_models.ClusterRemoveNodeIn,
) (*diom_models.ClusterRemoveNodeOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterRemoveNodeIn, diom_models.ClusterRemoveNodeOut](
		ctx,
		clusterAdmin.client,
		"POST",
		"/api/v1.cluster-admin.remove-node",
		&clusterRemoveNodeIn,
	)
}

// Force the cluster to take a snapshot immediately
func (clusterAdmin ClusterAdmin) ForceSnapshot(
	ctx context.Context,
	clusterForceSnapshotIn diom_models.ClusterForceSnapshotIn,
) (*diom_models.ClusterForceSnapshotOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterForceSnapshotIn, diom_models.ClusterForceSnapshotOut](
		ctx,
		clusterAdmin.client,
		"POST",
		"/api/v1.cluster-admin.force-snapshot",
		&clusterForceSnapshotIn,
	)
}
