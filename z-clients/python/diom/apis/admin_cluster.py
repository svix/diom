# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    ClusterForceSnapshotIn,
    ClusterForceSnapshotOut,
    ClusterInitializeIn,
    ClusterInitializeOut,
    ClusterRemoveNodeIn,
    ClusterRemoveNodeOut,
    ClusterStatusOut,
)


class AdminClusterAsync(ApiBase):
    async def status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        response = await self._request_asyncio(
            method="get",
            path="/api/v1.admin.cluster.status",
        )
        return parse_response(response, ClusterStatusOut)

    async def initialize(
        self,
        cluster_initialize_in: ClusterInitializeIn = ClusterInitializeIn(),
    ) -> ClusterInitializeOut:
        """Initialize this node as the leader of a new cluster

        This operation may only be performed against a node which has not been
        initialized and is not currently a member of a cluster."""
        body = cluster_initialize_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.cluster.initialize",
            body=body,
        )
        return parse_response(response, ClusterInitializeOut)

    async def remove_node(
        self,
        cluster_remove_node_in: ClusterRemoveNodeIn,
    ) -> ClusterRemoveNodeOut:
        """Remove a node from the cluster.

        This operation executes immediately and the node must be wiped and reset
        before it can safely be added to the cluster."""
        body = cluster_remove_node_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.cluster.remove-node",
            body=body,
        )
        return parse_response(response, ClusterRemoveNodeOut)

    async def force_snapshot(
        self,
        cluster_force_snapshot_in: ClusterForceSnapshotIn = ClusterForceSnapshotIn(),
    ) -> ClusterForceSnapshotOut:
        """Force the cluster to take a snapshot immediately"""
        body = cluster_force_snapshot_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.admin.cluster.force-snapshot",
            body=body,
        )
        return parse_response(response, ClusterForceSnapshotOut)


class AdminCluster(ApiBase):
    def status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        response = self._request_sync(
            method="get",
            path="/api/v1.admin.cluster.status",
        )
        return parse_response(response, ClusterStatusOut)

    def initialize(
        self,
        cluster_initialize_in: ClusterInitializeIn = ClusterInitializeIn(),
    ) -> ClusterInitializeOut:
        """Initialize this node as the leader of a new cluster

        This operation may only be performed against a node which has not been
        initialized and is not currently a member of a cluster."""
        body = cluster_initialize_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.cluster.initialize",
            body=body,
        )
        return parse_response(response, ClusterInitializeOut)

    def remove_node(
        self,
        cluster_remove_node_in: ClusterRemoveNodeIn,
    ) -> ClusterRemoveNodeOut:
        """Remove a node from the cluster.

        This operation executes immediately and the node must be wiped and reset
        before it can safely be added to the cluster."""
        body = cluster_remove_node_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.cluster.remove-node",
            body=body,
        )
        return parse_response(response, ClusterRemoveNodeOut)

    def force_snapshot(
        self,
        cluster_force_snapshot_in: ClusterForceSnapshotIn = ClusterForceSnapshotIn(),
    ) -> ClusterForceSnapshotOut:
        """Force the cluster to take a snapshot immediately"""
        body = cluster_force_snapshot_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.admin.cluster.force-snapshot",
            body=body,
        )
        return parse_response(response, ClusterForceSnapshotOut)
