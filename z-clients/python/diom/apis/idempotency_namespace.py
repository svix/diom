# This file is @generated

from ..internal.api_common import ApiBase, parse_response
from ..models import (
    IdempotencyCreateNamespaceIn,
    IdempotencyCreateNamespaceOut,
    IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceOut,
)


class IdempotencyNamespaceAsync(ApiBase):
    async def create(
        self,
        idempotency_create_namespace_in: IdempotencyCreateNamespaceIn,
    ) -> IdempotencyCreateNamespaceOut:
        """Create idempotency namespace"""
        body = idempotency_create_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.namespace.create",
            body=body,
        )
        return parse_response(response, IdempotencyCreateNamespaceOut)

    async def get(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        response = await self._request_asyncio(
            method="post",
            path="/api/v1.idempotency.namespace.get",
            body=body,
        )
        return parse_response(response, IdempotencyGetNamespaceOut)


class IdempotencyNamespace(ApiBase):
    def create(
        self,
        idempotency_create_namespace_in: IdempotencyCreateNamespaceIn,
    ) -> IdempotencyCreateNamespaceOut:
        """Create idempotency namespace"""
        body = idempotency_create_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.idempotency.namespace.create",
            body=body,
        )
        return parse_response(response, IdempotencyCreateNamespaceOut)

    def get(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(exclude_none=True)

        response = self._request_sync(
            method="post",
            path="/api/v1.idempotency.namespace.get",
            body=body,
        )
        return parse_response(response, IdempotencyGetNamespaceOut)
