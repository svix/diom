# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    PingOut,
    ReadyOut,
)


class HealthAsync(ApiBase):
    async def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running.

        This endpoint only checks the server itself, not the cluster mechanism, and should not be used
        as a readiness gate."""

        return await self._request_asyncio(
            method="get",
            path="/api/v1.health.ping",
            response_type=PingOut,
        )

    async def ready(
        self,
    ) -> ReadyOut:
        """Verify that this server is ready to serve customer traffic."""

        return await self._request_asyncio(
            method="get",
            path="/api/v1.health.ready",
            response_type=ReadyOut,
        )

    async def error(
        self,
    ) -> None:
        """Intentionally return an error"""

        await self._request_asyncio_no_response(
            method="post",
            path="/api/v1.health.error",
        )


class Health(ApiBase):
    def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running.

        This endpoint only checks the server itself, not the cluster mechanism, and should not be used
        as a readiness gate."""

        return self._request_sync(
            method="get",
            path="/api/v1.health.ping",
            response_type=PingOut,
        )

    def ready(
        self,
    ) -> ReadyOut:
        """Verify that this server is ready to serve customer traffic."""

        return self._request_sync(
            method="get",
            path="/api/v1.health.ready",
            response_type=ReadyOut,
        )

    def error(
        self,
    ) -> None:
        """Intentionally return an error"""

        self._request_sync_no_response(
            method="post",
            path="/api/v1.health.error",
        )
