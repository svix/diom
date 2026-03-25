# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    TransformIn,
    TransformOut,
)


class TransformationsAsync(ApiBase):
    async def execute(
        self,
        transform_in: TransformIn,
    ) -> TransformOut:
        """Execute a JavaScript transformation script against a payload and return the result."""
        body = transform_in.model_dump(exclude_none=True)

        return await self._request_asyncio(
            method="post",
            path="/api/v1.transformations.execute",
            body=body,
            response_type=TransformOut,
        )


class Transformations(ApiBase):
    def execute(
        self,
        transform_in: TransformIn,
    ) -> TransformOut:
        """Execute a JavaScript transformation script against a payload and return the result."""
        body = transform_in.model_dump(exclude_none=True)

        return self._request_sync(
            method="post",
            path="/api/v1.transformations.execute",
            body=body,
            response_type=TransformOut,
        )
