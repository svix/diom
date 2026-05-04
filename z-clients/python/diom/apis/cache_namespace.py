# This file is @generated

from ..internal.api_common import ApiBase
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

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cache.namespace.configure",
            body=body,
            response_type=CacheConfigureNamespaceOut,
        )

    async def get(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        body = cache_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.cache.namespace.get",
            body=body,
            response_type=CacheGetNamespaceOut,
        )


class CacheNamespace(ApiBase):
    def configure(
        self,
        cache_configure_namespace_in: CacheConfigureNamespaceIn,
    ) -> CacheConfigureNamespaceOut:
        """Configure cache namespace"""
        body = cache_configure_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cache.namespace.configure",
            body=body,
            response_type=CacheConfigureNamespaceOut,
        )

    def get(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        body = cache_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.cache.namespace.get",
            body=body,
            response_type=CacheGetNamespaceOut,
        )
