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
    KvSetIn,
)

TOKEN = os.environ["COYOTE_TOKEN"]
SERVER_URL = os.environ["COYOTE_SERVER_URL"]

pytestmark = pytest.mark.skipif(
    os.environ.get("COYOTE_INTEGRATION") != "1",
    reason="Set COYOTE_INTEGRATION=1 to run integration tests",
)

@pytest.fixture
def make_client() -> Coyote:
    return Coyote(TOKEN, CoyoteOptions(server_url=SERVER_URL))

@pytest.fixture
def make_async_client() -> CoyoteAsync:
    return CoyoteAsync(TOKEN, CoyoteOptions(server_url=SERVER_URL))


# --- Sync tests ---


def test_health_ping():
    client = make_client()
    client.health.ping()


def test_kv_set_get_delete():
    client = make_client()
    key = "python-sync-kv-key"
    value = b"python-sync-kv-value"

    # Set
    set_resp = client.kv.set(key, KvSetIn(value=value))
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


def test_cache_set_get_delete():
    client = make_client()
    key = "python-sync-cache-key"
    value = b"python-sync-cache-value"

    # Set
    client.cache.set(key, CacheSetIn(value=value, ttl_ms=60000))

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
async def test_health_ping_async():
    client = make_async_client()
    await client.health.ping()


@pytest.mark.asyncio
async def test_kv_set_get_delete_async():
    client = make_async_client()
    key = "python-async-kv-key"
    value = b"python-async-kv-value"

    # Set
    set_resp = await client.kv.set(key, KvSetIn(value=value))
    assert set_resp.success is True

    # Get
    get_resp = await client.kv.get(key, KvGetIn())
    assert get_resp.value == value

    # Delete
    del_resp = await client.kv.delete(key, KvDeleteIn())
    assert del_resp.success is True

    # Verify deleted
    get_resp = await client.kv.get(key, KvGetIn())
    assert get_resp.value is None


@pytest.mark.asyncio
async def test_cache_set_get_delete_async():
    client = make_async_client()
    key = "python-async-cache-key"
    value = b"python-async-cache-value"

    # Set
    await client.cache.set(key, CacheSetIn(value=value, ttl_ms=60000))

    # Get
    get_resp = await client.cache.get(key, CacheGetIn())
    assert get_resp.value == value

    # Delete
    del_resp = await client.cache.delete(key, CacheDeleteIn())
    assert del_resp.success is True

    # Verify deleted
    get_resp = await client.cache.get(key, CacheGetIn())
    assert get_resp.value is None
