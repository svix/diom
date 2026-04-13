# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    KvCreateNamespaceIn,
    KvCreateNamespaceOut,
    KvGetNamespaceIn,
    KvGetNamespaceOut,
)


class KvNamespaceAsync(ApiBase):
    async def create(
        self,
        kv_create_namespace_in: KvCreateNamespaceIn,
    ) -> KvCreateNamespaceOut:
        """Create KV namespace"""
        body = kv_create_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.kv.namespace.create",
            body=body,
        )
        return parse_response(response, KvCreateNamespaceOut)

    async def get(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        body = kv_get_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.kv.namespace.get",
            body=body,
        )
        return parse_response(response, KvGetNamespaceOut)


class KvNamespace(ApiBase):
    def create(
        self,
        kv_create_namespace_in: KvCreateNamespaceIn,
    ) -> KvCreateNamespaceOut:
        """Create KV namespace"""
        body = kv_create_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.kv.namespace.create",
            body=body,
        )
        return parse_response(response, KvCreateNamespaceOut)

    def get(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        body = kv_get_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.kv.namespace.get",
            body=body,
        )
        return parse_response(response, KvGetNamespaceOut)
