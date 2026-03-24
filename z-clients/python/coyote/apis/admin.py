# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    ClusterRemoveNodeIn,
    ClusterRemoveNodeOut,
)
from .admin_cluster import (
    AdminCluster,
    AdminClusterAsync,
)


class AdminAsync(ApiBase):
    @property
    def cluster(self) -> AdminClusterAsync:
        return AdminClusterAsync(self._client)

    async def cluster_remove_node(
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


class Admin(ApiBase):
    @property
    def cluster(self) -> AdminCluster:
        return AdminCluster(self._client)

    def cluster_remove_node(
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
