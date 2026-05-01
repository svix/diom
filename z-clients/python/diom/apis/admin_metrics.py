# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    GetMetricsOut,
)


class AdminMetricsAsync(ApiBase):
    async def get(
        self,
    ) -> GetMetricsOut:
        """Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver)"""

        return await self._request_asyncio(
            method="get",
            path="/api/v1.admin.metrics.get",
            response_type=GetMetricsOut,
        )


class AdminMetrics(ApiBase):
    def get(
        self,
    ) -> GetMetricsOut:
        """Dump the current metrics (which would otherwise be sent to the OTLP metrics receiver)"""

        return self._request_sync(
            method="get",
            path="/api/v1.admin.metrics.get",
            response_type=GetMetricsOut,
        )
