# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
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
