import asyncio
import os

import pytest

from coyote import Coyote, CoyoteAsync, CoyoteOptions
from coyote.models import (
    CacheDeleteIn,
    CacheGetIn,
    CacheSetIn,
    KvDeleteIn,
    KvGetIn,
)

TOKEN = os.environ["COYOTE_TOKEN"]
SERVER_URL = os.environ["COYOTE_SERVER_URL"]

pytestmark = pytest.mark.skipif(
    os.environ.get("COYOTE_INTEGRATION") != "1",
    reason="Set COYOTE_INTEGRATION=1 to run integration tests",
)

@pytest.fixture
def client() -> Coyote:
    return Coyote(TOKEN, CoyoteOptions(server_url=SERVER_URL))

@pytest.fixture
def async_client() -> CoyoteAsync:
    return CoyoteAsync(TOKEN, CoyoteOptions(server_url=SERVER_URL))


# --- Sync tests ---


def test_health_ping(client):
    client.health.ping()


def test_kv_set_get_delete(client):
    key = "python-sync-kv-key"
    value = b"python-sync-kv-value"

    # Set
    set_resp = client.kv.set(key, value)
    assert set_resp.success is True

    # Get
    get_resp = client.kv.get(key, KvGetIn())
    assert get_resp.value == value

    # Delete
    del_resp = client.kv.delete(key, KvDeleteIn())
    assert del_resp.success is True

    # Verify deleted
    get_resp = client.kv.get(key, KvGetIn())
    assert get_resp.value is None


def test_cache_set_get_delete(client):
    key = "python-sync-cache-key"
    value = b"python-sync-cache-value"

    # Set
    client.cache.set(key, value, CacheSetIn(ttl_ms=60000))

    # Get
    get_resp = client.cache.get(key, CacheGetIn())
    assert get_resp.value == value

    # Delete
    del_resp = client.cache.delete(key, CacheDeleteIn())
    assert del_resp.success is True

    # Verify deleted
    get_resp = client.cache.get(key, CacheGetIn())
    assert get_resp.value is None


# --- Async tests ---


@pytest.mark.asyncio
async def test_health_ping_async(async_client):
    await async_client.health.ping()


@pytest.mark.asyncio
async def test_kv_set_get_delete_async(async_client):
    key = "python-async-kv-key"
    value = b"python-async-kv-value"

    # Set
    set_resp = await async_client.kv.set(key, value)
    assert set_resp.success is True

    # Get
    get_resp = await async_client.kv.get(key, KvGetIn())
    assert get_resp.value == value

    # Delete
    del_resp = await async_client.kv.delete(key, KvDeleteIn())
    assert del_resp.success is True

    # Verify deleted
    get_resp = await async_client.kv.get(key, KvGetIn())
    assert get_resp.value is None


@pytest.mark.asyncio
async def test_cache_set_get_delete_async(async_client):
    key = "python-async-cache-key"
    value = b"python-async-cache-value"

    # Set
    await async_client.cache.set(key, value, CacheSetIn(ttl_ms=60000))

    # Get
    get_resp = await async_client.cache.get(key, CacheGetIn())
    assert get_resp.value == value

    # Delete
    del_resp = await async_client.cache.delete(key, CacheDeleteIn())
    assert del_resp.success is True

    # Verify deleted
    get_resp = await async_client.cache.get(key, CacheGetIn())
    assert get_resp.value is None
