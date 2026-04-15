# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    CacheConfigureNamespaceIn,
    CacheConfigureNamespaceOut,
    CacheGetNamespaceIn,
    CacheGetNamespaceOut,
)


class CacheNamespaceAsync(ApiBase):
    async def configure(
        self,
        cache_configure_namespace_in: CacheConfigureNamespaceIn,
    ) -> CacheConfigureNamespaceOut:
        """Configure cache namespace"""
        body = cache_configure_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.cache.namespace.configure",
            body=body,
        )
        return parse_response(response, CacheConfigureNamespaceOut)

    async def get(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        body = cache_get_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.cache.namespace.get",
            body=body,
        )
        return parse_response(response, CacheGetNamespaceOut)


class CacheNamespace(ApiBase):
    def configure(
        self,
        cache_configure_namespace_in: CacheConfigureNamespaceIn,
    ) -> CacheConfigureNamespaceOut:
        """Configure cache namespace"""
        body = cache_configure_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.cache.namespace.configure",
            body=body,
        )
        return parse_response(response, CacheConfigureNamespaceOut)

    def get(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        body = cache_get_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.cache.namespace.get",
            body=body,
        )
        return parse_response(response, CacheGetNamespaceOut)
