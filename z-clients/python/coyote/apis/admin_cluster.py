# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    ClusterRemoveNodeIn,
    ClusterRemoveNodeOut,
    ClusterStatusOut,
)


class AdminClusterAsync(ApiBase):
    async def status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        return await self._request_asyncio(
            method="get",
            path="/api/v1/admin/cluster/status",
            response_type=ClusterStatusOut,
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
            path="/api/v1/admin/cluster/remove-node",
            body=body,
            response_type=ClusterRemoveNodeOut,
        )


class AdminCluster(ApiBase):
    def status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        return self._request_sync(
            method="get",
            path="/api/v1/admin/cluster/status",
            response_type=ClusterStatusOut,
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
            path="/api/v1/admin/cluster/remove-node",
            body=body,
            response_type=ClusterRemoveNodeOut,
        )
