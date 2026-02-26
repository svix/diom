# this file is @generated


from .cache import Cache, CacheAsync

from .health import Health, HealthAsync

from .idempotency import Idempotency, IdempotencyAsync

from .kv import Kv, KvAsync

from .rate_limiter import RateLimiter, RateLimiterAsync

from .stream import Stream, StreamAsync

from .client import ClientBase


class DiomAsync(ClientBase):
    @property
    def cache(self) -> CacheAsync:
        return CacheAsync(self._client)

    @property
    def health(self) -> HealthAsync:
        return HealthAsync(self._client)

    @property
    def idempotency(self) -> IdempotencyAsync:
        return IdempotencyAsync(self._client)

    @property
    def kv(self) -> KvAsync:
        return KvAsync(self._client)

    @property
    def rate_limiter(self) -> RateLimiterAsync:
        return RateLimiterAsync(self._client)

    @property
    def stream(self) -> StreamAsync:
        return StreamAsync(self._client)


class Diom(ClientBase):
    @property
    def cache(self) -> Cache:
        return Cache(self._client)

    @property
    def health(self) -> Health:
        return Health(self._client)

    @property
    def idempotency(self) -> Idempotency:
        return Idempotency(self._client)

    @property
    def kv(self) -> Kv:
        return Kv(self._client)

    @property
    def rate_limiter(self) -> RateLimiter:
        return RateLimiter(self._client)

    @property
    def stream(self) -> Stream:
        return Stream(self._client)
