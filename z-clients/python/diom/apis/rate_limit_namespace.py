# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    RateLimitConfigureNamespaceIn,
    RateLimitConfigureNamespaceOut,
    RateLimitGetNamespaceIn,
    RateLimitGetNamespaceOut,
)


class RateLimitNamespaceAsync(ApiBase):
    async def configure(
        self,
        rate_limit_configure_namespace_in: RateLimitConfigureNamespaceIn,
    ) -> RateLimitConfigureNamespaceOut:
        """Configure rate limiter namespace"""
        body = rate_limit_configure_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.rate-limit.namespace.configure",
            body=body,
        )
        return parse_response(response, RateLimitConfigureNamespaceOut)

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
    def configure(
        self,
        rate_limit_configure_namespace_in: RateLimitConfigureNamespaceIn,
    ) -> RateLimitConfigureNamespaceOut:
        """Configure rate limiter namespace"""
        body = rate_limit_configure_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.rate-limit.namespace.configure",
            body=body,
        )
        return parse_response(response, RateLimitConfigureNamespaceOut)

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
