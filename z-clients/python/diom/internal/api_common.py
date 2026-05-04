import asyncio
import random
import logging
import time
import typing as t
import msgpack
import uuid
import httpx
from pydantic import BaseModel

from .errors import ConnError, ErrorBody, OtherError, raise_diom_error
from .http_client import AuthenticatedHttpClient


APPLICATION_MSGPACK = "application/msgpack"


class ApiBase:
    _client: AuthenticatedHttpClient
    _httpx_client: httpx.Client
    _httpx_async_client: httpx.AsyncClient

    def __init__(self, client: AuthenticatedHttpClient) -> None:
        self._client = client

        if self._client.proxy is not None:
            proxy_mounts = {
                "http://": httpx.HTTPTransport(proxy=httpx.Proxy(self._client.proxy)),
                "https://": httpx.HTTPTransport(proxy=httpx.Proxy(self._client.proxy)),
            }
            async_proxy_mounts = {
                "http://": httpx.AsyncHTTPTransport(
                    proxy=httpx.Proxy(self._client.proxy)
                ),
                "https://": httpx.AsyncHTTPTransport(
                    proxy=httpx.Proxy(self._client.proxy)
                ),
            }
        else:
            proxy_mounts = None
            async_proxy_mounts = None

        self._httpx_client = httpx.Client(
            mounts=proxy_mounts, cookies=self._client.get_cookies(), http2=True
        )
        self._httpx_async_client = httpx.AsyncClient(
            mounts=async_proxy_mounts, cookies=self._client.get_cookies(), http2=True
        )

    def _get_httpx_kwargs(
        self,
        method: str,
        path: str,
        *,
        header_params: t.Optional[t.Dict[str, str]],
        body: t.Optional[t.Any],
    ) -> t.Dict[str, t.Any]:
        url = f"{self._client.base_url}{path}"

        headers: t.Dict[str, str] = {
            **self._client.get_headers(),
            "diom-req-id": f"{random.getrandbits(64)}",
            "accept": APPLICATION_MSGPACK,
        }
        if header_params is not None:
            headers.update(header_params)

        if headers.get("idempotency-key") is None and method.upper() == "POST":
            headers["idempotency-key"] = f"auto_{uuid.uuid4()}"

        httpx_kwargs: dict[str, t.Any] = {
            "method": method.upper(),
            "url": url,
            "headers": headers,
            "timeout": self._client.get_timeout(),
            "follow_redirects": self._client.follow_redirects,
        }

        if body is not None:
            # pyrefly: ignore
            encoded_body: bytes = msgpack.packb(body, strict_types=True)
            httpx_kwargs["content"] = encoded_body
            headers["content-type"] = APPLICATION_MSGPACK
            headers["content-length"] = str(len(encoded_body))

        return httpx_kwargs

    async def _request_asyncio[T: BaseModel](
        self,
        method: str,
        path: str,
        *,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
        response_type: type[T],
    ) -> T:
        op_id = op_id_from_path(path)
        response = await self._request_asyncio_inner(
            method, path, op_id, header_params, body
        )
        return parse_response(response, response_type, op_id)

    async def _request_asyncio_no_response(
        self,
        method: str,
        path: str,
        *,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
    ) -> None:
        op_id = op_id_from_path(path)
        response = await self._request_asyncio_inner(
            method, path, op_id, header_params, body
        )
        check_response(response, op_id)

    def _request_sync[T: BaseModel](
        self,
        method: str,
        path: str,
        *,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
        response_type: type[T],
    ) -> T:
        op_id = op_id_from_path(path)
        response = self._request_sync_inner(method, path, op_id, header_params, body)
        return parse_response(response, response_type, op_id)

    def _request_sync_no_response(
        self,
        method: str,
        path: str,
        *,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
    ) -> None:
        op_id = op_id_from_path(path)
        response = self._request_sync_inner(method, path, op_id, header_params, body)
        check_response(response, op_id)

    async def _request_asyncio_inner(
        self,
        method: str,
        path: str,
        op_id: str,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
    ) -> httpx.Response:
        try:
            httpx_kwargs = self._get_httpx_kwargs(
                method,
                path,
                header_params=header_params,
                body=body,
            )
        except Exception:
            raise OtherError(op_id)

        try:
            response = await self._httpx_async_client.request(**httpx_kwargs)
        except Exception:
            raise ConnError(op_id)

        for retry_count, sleep_time in enumerate(self._client.retry_schedule):
            if response.status_code < 500:
                break

            await asyncio.sleep(sleep_time)
            httpx_kwargs["headers"]["diom-retry-count"] = str(retry_count)
            try:
                response = await self._httpx_async_client.request(**httpx_kwargs)
            except Exception:
                raise ConnError(op_id)

        return response

    def _request_sync_inner(
        self,
        method: str,
        path: str,
        op_id: str,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
    ) -> httpx.Response:
        try:
            httpx_kwargs = self._get_httpx_kwargs(
                method,
                path,
                header_params=header_params,
                body=body,
            )
        except Exception:
            raise OtherError(op_id)

        try:
            response = self._httpx_client.request(**httpx_kwargs)
        except Exception:
            raise ConnError(op_id)

        for retry_count, sleep_time in enumerate(self._client.retry_schedule):
            if response.status_code < 500:
                break

            time.sleep(sleep_time)
            httpx_kwargs["headers"]["diom-retry-count"] = str(retry_count)
            try:
                response = self._httpx_client.request(**httpx_kwargs)
            except Exception:
                raise ConnError(op_id)

        return response


def op_id_from_path(path: str) -> str:
    if path.startswith("/api/"):
        return path.removeprefix("/api/")
    else:
        logging.error("request path must begin with /api/")
        return "[ERROR]"


def decode_response_body(response: httpx.Response, op_id: str):
    content_type = response.headers.get("content-type", "application/json")
    try:
        if content_type == "application/msgpack":
            return msgpack.unpackb(response.content)
        else:
            return response.json()
    except Exception:
        raise OtherError(op_id)


def check_response(response: httpx.Response, op_id: str) -> None:
    if response.status_code >= 300:
        error_body = _parse_response(response, ErrorBody, op_id)
        raise_diom_error(error_body, op_id)


def parse_response[T: BaseModel](
    response: httpx.Response,
    response_type: type[T],
    op_id: str,
) -> T:
    check_response(response, op_id)
    return _parse_response(response, response_type, op_id)


def _parse_response[T: BaseModel](
    response: httpx.Response,
    response_type: type[T],
    op_id: str,
) -> T:
    try:
        return response_type.model_validate(
            decode_response_body(response, op_id),
            by_alias=True,
            by_name=False,
        )
    except Exception:
        raise OtherError(op_id)
