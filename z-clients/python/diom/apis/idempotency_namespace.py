# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    IdempotencyConfigureNamespaceIn,
    IdempotencyConfigureNamespaceOut,
    IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceOut,
)


class IdempotencyNamespaceAsync(ApiBase):
    async def configure(
        self,
        idempotency_configure_namespace_in: IdempotencyConfigureNamespaceIn,
    ) -> IdempotencyConfigureNamespaceOut:
        """Configure idempotency namespace"""
        body = idempotency_configure_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.namespace.configure",
            body=body,
            response_type=IdempotencyConfigureNamespaceOut,
        )

    async def get(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.namespace.get",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )


class IdempotencyNamespace(ApiBase):
    def configure(
        self,
        idempotency_configure_namespace_in: IdempotencyConfigureNamespaceIn,
    ) -> IdempotencyConfigureNamespaceOut:
        """Configure idempotency namespace"""
        body = idempotency_configure_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.idempotency.namespace.configure",
            body=body,
            response_type=IdempotencyConfigureNamespaceOut,
        )

    def get(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.idempotency.namespace.get",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )
