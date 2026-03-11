# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    RateLimiterCreateNamespaceIn,
    RateLimiterCreateNamespaceOut,
    RateLimiterGetNamespaceIn,
    RateLimiterGetNamespaceOut,
)


class RateLimiterNamespaceAsync(ApiBase):
    async def create(
        self,
        rate_limiter_create_namespace_in: RateLimiterCreateNamespaceIn,
    ) -> RateLimiterCreateNamespaceOut:
        """Create rate limiter namespace"""
        body = rate_limiter_create_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limit/namespace/create",
            body=body,
            response_type=RateLimiterCreateNamespaceOut,
        )

    async def get(
        self,
        rate_limiter_get_namespace_in: RateLimiterGetNamespaceIn,
    ) -> RateLimiterGetNamespaceOut:
        """Get rate limiter namespace"""
        body = rate_limiter_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limit/namespace/get",
            body=body,
            response_type=RateLimiterGetNamespaceOut,
        )


class RateLimiterNamespace(ApiBase):
    def create(
        self,
        rate_limiter_create_namespace_in: RateLimiterCreateNamespaceIn,
    ) -> RateLimiterCreateNamespaceOut:
        """Create rate limiter namespace"""
        body = rate_limiter_create_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/rate-limit/namespace/create",
            body=body,
            response_type=RateLimiterCreateNamespaceOut,
        )

    def get(
        self,
        rate_limiter_get_namespace_in: RateLimiterGetNamespaceIn,
    ) -> RateLimiterGetNamespaceOut:
        """Get rate limiter namespace"""
        body = rate_limiter_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1/rate-limit/namespace/get",
            body=body,
            response_type=RateLimiterGetNamespaceOut,
        )
