import httpx
import pytest

from diom import Diom, DiomOptions
from diom.internal.errors import ConnError, OtherError


def test_retries_on_5xx(httpx_mock):
    diom = Diom(
        "not-a-real-api-key",
        DiomOptions(server_url="https://diom.example", retry_schedule=[0.1, 0.1]),
    )
    calls = []

    def error_but_only_once(request: httpx.Request):
        nonlocal calls
        calls.append(request)
        if len(calls) < 2:
            return httpx.Response(
                status_code=500,
                json={
                    "code": "INTERNAL_SERVER_ERROR",
                    "detail": "oopsie we did a moopsie",
                },
            )
        return httpx.Response(
            status_code=200,
            json={"ok": True},
        )

    httpx_mock.add_callback(
        error_but_only_once,
        url="https://diom.example/api/v1.health.ping",
        method="GET",
        is_reusable=True,
    )

    resp = diom.health.ping()
    assert resp.ok


def test_retries_on_timeout(httpx_mock):
    diom = Diom(
        "not-a-real-api-key",
        DiomOptions(server_url="https://diom.example", retry_schedule=[0.1, 0.1]),
    )
    calls = []

    def timeout_but_only_once(request: httpx.Request):
        nonlocal calls
        calls.append(request)
        if len(calls) < 2:
            raise httpx.ReadTimeout("timed out reading from https://diom.example")
        return httpx.Response(
            status_code=200,
            json={"ok": True},
        )

    httpx_mock.add_callback(
        timeout_but_only_once,
        url="https://diom.example/api/v1.health.ping",
        method="GET",
        is_reusable=True,
    )

    resp = diom.health.ping()
    assert resp.ok


def test_does_not_retry_on_400(httpx_mock):
    diom = Diom(
        "not-a-real-api-key",
        DiomOptions(server_url="https://diom.example", retry_schedule=[0.1, 0.1]),
    )

    httpx_mock.add_response(
        url="https://diom.example/api/v1.health.ping",
        method="GET",
        status_code=400,
        json={"code": "BAD_REQUEST", "detail": "naughty, dirty little request"},
    )

    with pytest.raises(OtherError):
        diom.health.ping()


def test_handles_retry_exhaustion_for_timeouts(httpx_mock):
    diom = Diom(
        "not-a-real-api-key",
        DiomOptions(server_url="https://diom.example", retry_schedule=[0.1, 0.1]),
    )

    def fail_immediately():
        raise httpx.ReadTimeout("timed out reading")

    httpx_mock.add_callback(
        fail_immediately,
        url="https://diom.example/api/v1.health.ping",
        method="GET",
        is_reusable=True,
    )

    with pytest.raises(ConnError):
        diom.health.ping()


def test_handles_retry_exhaustion_for_5xx(httpx_mock):
    diom = Diom(
        "not-a-real-api-key",
        DiomOptions(server_url="https://diom.example", retry_schedule=[0.1, 0.1]),
    )

    httpx_mock.add_response(
        url="https://diom.example/api/v1.health.ping",
        method="GET",
        status_code=500,
        json={
            "code": "INTERNAL_SERVER_ERROR",
            "detail": "oopsie we did a moopsie",
        },
        is_reusable=True,
    )

    with pytest.raises(OtherError):
        diom.health.ping()
