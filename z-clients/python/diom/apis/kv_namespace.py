# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    KvConfigureNamespaceIn,
    KvConfigureNamespaceOut,
    KvGetNamespaceIn,
    KvGetNamespaceOut,
)


class KvNamespaceAsync(ApiBase):
    async def configure(
        self,
        kv_configure_namespace_in: KvConfigureNamespaceIn,
    ) -> KvConfigureNamespaceOut:
        """Configure KV namespace"""
        body = kv_configure_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.kv.namespace.configure",
            body=body,
            response_type=KvConfigureNamespaceOut,
        )

    async def get(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        body = kv_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.kv.namespace.get",
            body=body,
            response_type=KvGetNamespaceOut,
        )


class KvNamespace(ApiBase):
    def configure(
        self,
        kv_configure_namespace_in: KvConfigureNamespaceIn,
    ) -> KvConfigureNamespaceOut:
        """Configure KV namespace"""
        body = kv_configure_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.kv.namespace.configure",
            body=body,
            response_type=KvConfigureNamespaceOut,
        )

    def get(
        self,
        kv_get_namespace_in: KvGetNamespaceIn,
    ) -> KvGetNamespaceOut:
        """Get KV namespace"""
        body = kv_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.kv.namespace.get",
            body=body,
            response_type=KvGetNamespaceOut,
        )
