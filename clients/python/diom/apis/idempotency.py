# This file is @generated

from .common import ApiBase
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
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/idempotency/abort",
            path_params={},
            json_body=idempotency_abort_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IdempotencyAbortOut.model_validate(response.json())

    async def get_namespace(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/idempotency/get-namespace",
            path_params={},
            json_body=idempotency_get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IdempotencyGetNamespaceOut.model_validate(response.json())


class Idempotency(ApiBase):
    def abort(
        self,
        idempotency_abort_in: IdempotencyAbortIn,
    ) -> IdempotencyAbortOut:
        """Abandon an idempotent request (remove lock without saving response)"""
        response = self._request_sync(
            method="post",
            path="/api/v1/idempotency/abort",
            path_params={},
            json_body=idempotency_abort_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IdempotencyAbortOut.model_validate(response.json())

    def get_namespace(
        self,
        idempotency_get_namespace_in: IdempotencyGetNamespaceIn,
    ) -> IdempotencyGetNamespaceOut:
        """Get idempotency namespace"""
        response = self._request_sync(
            method="post",
            path="/api/v1/idempotency/get-namespace",
            path_params={},
            json_body=idempotency_get_namespace_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return IdempotencyGetNamespaceOut.model_validate(response.json())
