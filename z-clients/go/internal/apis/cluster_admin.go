package diom_apis

// This file is @generated DO NOT EDIT

import (
	"context"

	diom_models "diom.com/go/diom/internal/models"
	diom_proto "diom.com/go/diom/internal/proto"
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

// Force the cluster to conduct an election immediately
func (clusterAdmin ClusterAdmin) ForceElection(
	ctx context.Context,
	clusterForceElectionIn diom_models.ClusterForceElectionIn,
) (*diom_models.ClusterForceElectionOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterForceElectionIn, diom_models.ClusterForceElectionOut](
		ctx,
		clusterAdmin.client,
		"POST",
		"/api/v1.cluster-admin.force-election",
		&clusterForceElectionIn,
	)
}

// Request that the cluster upgrade the given node from "learner" to "voter"
//
// This should only be invoked if a partition occurs during a learner process
// and you don't want to re-bootstrap the affected node.
func (clusterAdmin ClusterAdmin) ForceNodeUpgrade(
	ctx context.Context,
	clusterForceNodeUpgradeIn diom_models.ClusterForceNodeUpgradeIn,
) (*diom_models.ClusterForceNodeUpgradeOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterForceNodeUpgradeIn, diom_models.ClusterForceNodeUpgradeOut](
		ctx,
		clusterAdmin.client,
		"POST",
		"/api/v1.cluster-admin.force-node-upgrade",
		&clusterForceNodeUpgradeIn,
	)
}

// Request that the cluster upgrade the given node from "voter" to "learner"
//
// This should only be invoked if you are testing the replication system
func (clusterAdmin ClusterAdmin) ForceNodeDowngrade(
	ctx context.Context,
	clusterForceNodeDowngradeIn diom_models.ClusterForceNodeDowngradeIn,
) (*diom_models.ClusterForceNodeDowngradeOut, error) {
	return diom_proto.ExecuteRequest[diom_models.ClusterForceNodeDowngradeIn, diom_models.ClusterForceNodeDowngradeOut](
		ctx,
		clusterAdmin.client,
		"POST",
		"/api/v1.cluster-admin.force-node-downgrade",
		&clusterForceNodeDowngradeIn,
	)
}
