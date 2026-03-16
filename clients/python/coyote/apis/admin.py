# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    ClusterStatusOut,
)


class AdminAsync(ApiBase):
    async def cluster_status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        return await self._request_asyncio(
            method="get",
            path="/api/v1/admin/cluster-status",
            response_type=ClusterStatusOut,
        )


class Admin(ApiBase):
    def cluster_status(
        self,
    ) -> ClusterStatusOut:
        """Get information about the current cluster"""

        return self._request_sync(
            method="get",
            path="/api/v1/admin/cluster-status",
            response_type=ClusterStatusOut,
        )
