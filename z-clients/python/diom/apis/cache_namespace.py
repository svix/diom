# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    CacheCreateNamespaceIn,
    CacheCreateNamespaceOut,
    CacheGetNamespaceIn,
    CacheGetNamespaceOut,
)


class CacheNamespaceAsync(ApiBase):
    async def create(
        self,
        cache_create_namespace_in: CacheCreateNamespaceIn,
    ) -> CacheCreateNamespaceOut:
        """Create cache namespace"""
        body = cache_create_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.cache.namespace.create",
            body=body,
        )
        return parse_response(response, CacheCreateNamespaceOut)

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
    def create(
        self,
        cache_create_namespace_in: CacheCreateNamespaceIn,
    ) -> CacheCreateNamespaceOut:
        """Create cache namespace"""
        body = cache_create_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.cache.namespace.create",
            body=body,
        )
        return parse_response(response, CacheCreateNamespaceOut)

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
