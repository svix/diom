# This file is @generated

from ..internal.api_common import ApiBase, check_response, parse_response
from ..models import (
    PingOut,
)


class HealthAsync(ApiBase):
    async def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running."""

        response = await self._request_asyncio(
            method="get",
            path="/api/v1.health.ping",
        )
        return parse_response(response, PingOut)

    async def error(
        self,
    ) -> None:
        """Intentionally return an error"""

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.health.error",
        )
        check_response(response)


class Health(ApiBase):
    def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running."""

        response = self._request_sync(
            method="get",
            path="/api/v1.health.ping",
        )
        return parse_response(response, PingOut)

    def error(
        self,
    ) -> None:
        """Intentionally return an error"""

        response = self._request_sync(
            method="post",
            path="/api/v1.health.error",
        )
        check_response(response)
