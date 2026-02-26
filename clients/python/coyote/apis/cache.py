# This file is @generated

from .common import ApiBase
from ..models import (
    CacheDeleteIn,
    CacheDeleteOut,
    CacheGetIn,
    CacheGetNamespaceIn,
    CacheGetNamespaceOut,
    CacheGetOut,
    CacheSetIn,
    CacheSetOut,
)


class CacheAsync(ApiBase):
    async def set(
        self,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/cache/set",
            path_params={},
            json_body=cache_set_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return CacheSetOut.model_validate(response.json())

    async def get(
        self,
        cache_get_in: CacheGetIn,
    ) -> CacheGetOut:
        """Cache Get"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/cache/get",
            path_params={},
            json_body=cache_get_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return CacheGetOut.model_validate(response.json())

    async def get_namespace(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/cache/get-namespace",
            path_params={},
            json_body=cache_get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CacheGetNamespaceOut.model_validate(response.json())

    async def delete(
        self,
        cache_delete_in: CacheDeleteIn,
    ) -> CacheDeleteOut:
        """Cache Delete"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/cache/delete",
            path_params={},
            json_body=cache_delete_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CacheDeleteOut.model_validate(response.json())


class Cache(ApiBase):
    def set(
        self,
        cache_set_in: CacheSetIn,
    ) -> CacheSetOut:
        """Cache Set"""
        response = self._request_sync(
            method="post",
            path="/api/v1/cache/set",
            path_params={},
            json_body=cache_set_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return CacheSetOut.model_validate(response.json())

    def get(
        self,
        cache_get_in: CacheGetIn,
    ) -> CacheGetOut:
        """Cache Get"""
        response = self._request_sync(
            method="post",
            path="/api/v1/cache/get",
            path_params={},
            json_body=cache_get_in.model_dump_json(exclude_unset=True, by_alias=True),
        )
        return CacheGetOut.model_validate(response.json())

    def get_namespace(
        self,
        cache_get_namespace_in: CacheGetNamespaceIn,
    ) -> CacheGetNamespaceOut:
        """Get cache namespace"""
        response = self._request_sync(
            method="post",
            path="/api/v1/cache/get-namespace",
            path_params={},
            json_body=cache_get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CacheGetNamespaceOut.model_validate(response.json())

    def delete(
        self,
        cache_delete_in: CacheDeleteIn,
    ) -> CacheDeleteOut:
        """Cache Delete"""
        response = self._request_sync(
            method="post",
            path="/api/v1/cache/delete",
            path_params={},
            json_body=cache_delete_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return CacheDeleteOut.model_validate(response.json())
