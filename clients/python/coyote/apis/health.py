# This file is @generated

from .common import ApiBase
from ..models import (
    PingOut,
)


class HealthAsync(ApiBase):
    async def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running."""
        response = await self._request_asyncio(
            method="get", path="/api/v1/health/ping", path_params={}
        )
        return PingOut.model_validate(response.json())


class Health(ApiBase):
    def ping(
        self,
    ) -> PingOut:
        """Verify the server is up and running."""
        response = self._request_sync(
            method="get", path="/api/v1/health/ping", path_params={}
        )
        return PingOut.model_validate(response.json())
