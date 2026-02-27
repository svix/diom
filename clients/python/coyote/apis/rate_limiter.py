# This file is @generated

from ..internal.api_common import ApiBase
from ..models import (
    RateLimiterCheckIn,
    RateLimiterCheckOut,
    RateLimiterGetRemainingIn,
    RateLimiterGetRemainingOut,
)


class RateLimiterAsync(ApiBase):
    async def limit(
        self,
        rate_limiter_check_in: RateLimiterCheckIn,
    ) -> RateLimiterCheckOut:
        """Rate Limiter Check and Consume"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limiter/limit",
            path_params={},
            json_body=rate_limiter_check_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return RateLimiterCheckOut.model_validate(response.json())

    async def get_remaining(
        self,
        rate_limiter_get_remaining_in: RateLimiterGetRemainingIn,
    ) -> RateLimiterGetRemainingOut:
        """Rate Limiter Get Remaining"""
        response = await self._request_asyncio(
            method="post",
            path="/api/v1/rate-limiter/get-remaining",
            path_params={},
            json_body=rate_limiter_get_remaining_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return RateLimiterGetRemainingOut.model_validate(response.json())


class RateLimiter(ApiBase):
    def limit(
        self,
        rate_limiter_check_in: RateLimiterCheckIn,
    ) -> RateLimiterCheckOut:
        """Rate Limiter Check and Consume"""
        response = self._request_sync(
            method="post",
            path="/api/v1/rate-limiter/limit",
            path_params={},
            json_body=rate_limiter_check_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return RateLimiterCheckOut.model_validate(response.json())

    def get_remaining(
        self,
        rate_limiter_get_remaining_in: RateLimiterGetRemainingIn,
    ) -> RateLimiterGetRemainingOut:
        """Rate Limiter Get Remaining"""
        response = self._request_sync(
            method="post",
            path="/api/v1/rate-limiter/get-remaining",
            path_params={},
            json_body=rate_limiter_get_remaining_in.model_dump_json(
                exclude_unset=True, by_alias=True
            ),
        )
        return RateLimiterGetRemainingOut.model_validate(response.json())
