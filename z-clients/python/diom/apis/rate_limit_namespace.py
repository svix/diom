# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    RateLimitCreateNamespaceIn,
    RateLimitCreateNamespaceOut,
    RateLimitGetNamespaceIn,
    RateLimitGetNamespaceOut,
)


class RateLimitNamespaceAsync(ApiBase):
    async def create(
        self,
        rate_limit_create_namespace_in: RateLimitCreateNamespaceIn,
    ) -> RateLimitCreateNamespaceOut:
        """Create rate limiter namespace"""
        body = rate_limit_create_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.rate-limit.namespace.create",
            body=body,
        )
        return parse_response(response, RateLimitCreateNamespaceOut)

    async def get(
        self,
        rate_limit_get_namespace_in: RateLimitGetNamespaceIn,
    ) -> RateLimitGetNamespaceOut:
        """Get rate limiter namespace"""
        body = rate_limit_get_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.rate-limit.namespace.get",
            body=body,
        )
        return parse_response(response, RateLimitGetNamespaceOut)


class RateLimitNamespace(ApiBase):
    def create(
        self,
        rate_limit_create_namespace_in: RateLimitCreateNamespaceIn,
    ) -> RateLimitCreateNamespaceOut:
        """Create rate limiter namespace"""
        body = rate_limit_create_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.rate-limit.namespace.create",
            body=body,
        )
        return parse_response(response, RateLimitCreateNamespaceOut)

    def get(
        self,
        rate_limit_get_namespace_in: RateLimitGetNamespaceIn,
    ) -> RateLimitGetNamespaceOut:
        """Get rate limiter namespace"""
        body = rate_limit_get_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.rate-limit.namespace.get",
            body=body,
        )
        return parse_response(response, RateLimitGetNamespaceOut)
