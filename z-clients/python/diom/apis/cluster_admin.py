# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    ClusterForceElectionIn,
    ClusterForceElectionOut,
    ClusterForceSnapshotIn,
    ClusterForceSnapshotOut,
    ClusterInitializeIn,
    ClusterInitializeOut,
    ClusterRemoveNodeIn,
    ClusterRemoveNodeOut,
    ClusterStatusOut,
)


class ClusterAdminAsync(ApiBase):
    async def status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        return await self._request_asyncio(
            method="get",
            path="/api/v1.cluster-admin.status",
            response_type=ClusterStatusOut,
        )

    async def initialize(
        self,
        cluster_initialize_in: ClusterInitializeIn = ClusterInitializeIn(),
    ) -> ClusterInitializeOut:
        """Initialize this node as the leader of a new cluster

        This operation may only be performed against a node which has not been
        initialized and is not currently a member of a cluster."""
        body = cluster_initialize_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cluster-admin.initialize",
            body=body,
            response_type=ClusterInitializeOut,
        )

    async def remove_node(
        self,
        cluster_remove_node_in: ClusterRemoveNodeIn,
    ) -> ClusterRemoveNodeOut:
        """Remove a node from the cluster.

        This operation executes immediately and the node must be wiped and reset
        before it can safely be added to the cluster."""
        body = cluster_remove_node_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cluster-admin.remove-node",
            body=body,
            response_type=ClusterRemoveNodeOut,
        )

    async def force_snapshot(
        self,
        cluster_force_snapshot_in: ClusterForceSnapshotIn = ClusterForceSnapshotIn(),
    ) -> ClusterForceSnapshotOut:
        """Force the cluster to take a snapshot immediately"""
        body = cluster_force_snapshot_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cluster-admin.force-snapshot",
            body=body,
            response_type=ClusterForceSnapshotOut,
        )

    async def force_election(
        self,
        cluster_force_election_in: ClusterForceElectionIn = ClusterForceElectionIn(),
    ) -> ClusterForceElectionOut:
        """Force the cluster to conduct an election immediately"""
        body = cluster_force_election_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cluster-admin.force-election",
            body=body,
            response_type=ClusterForceElectionOut,
        )


class ClusterAdmin(ApiBase):
    def status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        return self._request_sync(
            method="get",
            path="/api/v1.cluster-admin.status",
            response_type=ClusterStatusOut,
        )

    def initialize(
        self,
        cluster_initialize_in: ClusterInitializeIn = ClusterInitializeIn(),
    ) -> ClusterInitializeOut:
        """Initialize this node as the leader of a new cluster

        This operation may only be performed against a node which has not been
        initialized and is not currently a member of a cluster."""
        body = cluster_initialize_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cluster-admin.initialize",
            body=body,
            response_type=ClusterInitializeOut,
        )

    def remove_node(
        self,
        cluster_remove_node_in: ClusterRemoveNodeIn,
    ) -> ClusterRemoveNodeOut:
        """Remove a node from the cluster.

        This operation executes immediately and the node must be wiped and reset
        before it can safely be added to the cluster."""
        body = cluster_remove_node_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cluster-admin.remove-node",
            body=body,
            response_type=ClusterRemoveNodeOut,
        )

    def force_snapshot(
        self,
        cluster_force_snapshot_in: ClusterForceSnapshotIn = ClusterForceSnapshotIn(),
    ) -> ClusterForceSnapshotOut:
        """Force the cluster to take a snapshot immediately"""
        body = cluster_force_snapshot_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cluster-admin.force-snapshot",
            body=body,
            response_type=ClusterForceSnapshotOut,
        )

    def force_election(
        self,
        cluster_force_election_in: ClusterForceElectionIn = ClusterForceElectionIn(),
    ) -> ClusterForceElectionOut:
        """Force the cluster to conduct an election immediately"""
        body = cluster_force_election_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cluster-admin.force-election",
            body=body,
            response_type=ClusterForceElectionOut,
        )
