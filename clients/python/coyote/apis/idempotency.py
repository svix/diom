# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    IdempotencyAbortIn,
    IdempotencyAbortOut,
    IdempotencyGetNamespaceIn,
    IdempotencyGetNamespaceOut,
)


class IdempotencyAsync(ApiBase):
    async def abort(
        self,
        idempotency_abort_in: IdempotencyAbortIn,
    ) -> IdempotencyAbortOut:
        """Abandon an idempotent request (remove lock without saving response)"""
        body = idempotency_abort_in.model_dump(exclude_unset=True, by_alias=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1/idempotency/abort",
            body=body,
            response_type=IdempotencyAbortOut,
        )

    async def get_namespace(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(
            exclude_unset=True, by_alias=True
        )

        return await self._request_asyncio(
            method="post",
            path="/api/v1/idempotency/get-namespace",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )


class Idempotency(ApiBase):
    def abort(
        self,
        idempotency_abort_in: IdempotencyAbortIn,
    ) -> IdempotencyAbortOut:
        """Abandon an idempotent request (remove lock without saving response)"""
        body = idempotency_abort_in.model_dump(exclude_unset=True, by_alias=True)

        return self._request_sync(
            method="post",
            path="/api/v1/idempotency/abort",
            body=body,
            response_type=IdempotencyAbortOut,
        )

    def get_namespace(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        body = idempotency_get_namespace_in.model_dump(
            exclude_unset=True, by_alias=True
        )

        return self._request_sync(
            method="post",
            path="/api/v1/idempotency/get-namespace",
            body=body,
            response_type=IdempotencyGetNamespaceOut,
        )
