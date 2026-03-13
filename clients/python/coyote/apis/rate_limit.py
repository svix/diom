# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    RateLimitCheckIn,
    RateLimitCheckOut,
    RateLimitGetRemainingIn,
    RateLimitGetRemainingOut,
)
from .rate_limit_namespace import (
    RateLimitNamespace,
    RateLimitNamespaceAsync,
)


class RateLimitAsync(ApiBase):
    @property
    def namespace(self) -> RateLimitNamespaceAsync:
        return RateLimitNamespaceAsync(self._client)

    async def limit(
        self,
        rate_limit_check_in: RateLimitCheckIn,
    ) -> RateLimitCheckOut:
        """Rate Limiter Check and Consume"""
        body = rate_limit_check_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limit/limit",
            body=body,
            response_type=RateLimitCheckOut,
        )

    async def get_remaining(
        self,
        rate_limit_get_remaining_in: RateLimitGetRemainingIn,
    ) -> RateLimitGetRemainingOut:
        """Rate Limiter Get Remaining"""
        body = rate_limit_get_remaining_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limit/get-remaining",
            body=body,
            response_type=RateLimitGetRemainingOut,
        )


class RateLimit(ApiBase):
    @property
    def namespace(self) -> RateLimitNamespace:
        return RateLimitNamespace(self._client)

    def limit(
        self,
        rate_limit_check_in: RateLimitCheckIn,
    ) -> RateLimitCheckOut:
        """Rate Limiter Check and Consume"""
        body = rate_limit_check_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/rate-limit/limit",
            body=body,
            response_type=RateLimitCheckOut,
        )

    def get_remaining(
        self,
        rate_limit_get_remaining_in: RateLimitGetRemainingIn,
    ) -> RateLimitGetRemainingOut:
        """Rate Limiter Get Remaining"""
        body = rate_limit_get_remaining_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/rate-limit/get-remaining",
            body=body,
            response_type=RateLimitGetRemainingOut,
        )
