import asyncio
import random
import time
import typing as t
import msgpack
import uuid
import httpx
from pydantic import BaseModel

from .http_client import AuthenticatedHttpClient
from .errors.http_error import HttpError
from .errors.http_validation_error import HttpValidationError


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

        httpx_kwargs = {
            "method": method.upper(),
            "url": url,
            "headers": headers,
            "timeout": self._client.get_timeout(),
            "follow_redirects": self._client.follow_redirects,
        }

        if body is not None:
            encoded_body = msgpack.packb(body, strict_types=True)
            httpx_kwargs["content"] = encoded_body
            headers["content-type"] = APPLICATION_MSGPACK
            headers["content-length"] = str(len(encoded_body))

        return httpx_kwargs

    async def _request_asyncio[T: BaseModel](
        self,
        method: str,
        path: str,
        response_type: type[T],
        *,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
    ) -> T:
        httpx_kwargs = self._get_httpx_kwargs(
            method,
            path,
            header_params=header_params,
            body=body,
        )

        response = await self._httpx_async_client.request(**httpx_kwargs)

        for retry_count, sleep_time in enumerate(self._client.retry_schedule):
            if response.status_code < 500:
                break

            await asyncio.sleep(sleep_time)
            httpx_kwargs["headers"]["diom-retry-count"] = str(retry_count)
            response = await self._httpx_async_client.request(**httpx_kwargs)

        return _parse_response(response, response_type)

    def _request_sync[T: BaseModel](
        self,
        method: str,
        path: str,
        response_type: type[T],
        *,
        header_params: t.Optional[t.Dict[str, str]] = None,
        body: t.Optional[t.Any] = None,
    ) -> T:
        httpx_kwargs = self._get_httpx_kwargs(
            method,
            path,
            header_params=header_params,
            body=body,
        )
        response = self._httpx_client.request(**httpx_kwargs)
        for retry_count, sleep_time in enumerate(self._client.retry_schedule):
            if response.status_code < 500:
                break

            time.sleep(sleep_time)
            httpx_kwargs["headers"]["diom-retry-count"] = str(retry_count)
            response = self._httpx_client.request(**httpx_kwargs)

        return _parse_response(response, response_type)


def decode_response_body(response: httpx.Response):
    content_type = response.headers.get("content-type", "application/json")
    if content_type == "application/msgpack":
        return msgpack.unpackb(response.content)
    else:
        return response.json()


def _parse_response[T: BaseModel](
    response: httpx.Response,
    response_type: type[T],
) -> T:
    if 200 <= response.status_code <= 299:
        response_decoded = decode_response_body(response)
        return response_type.model_validate(
            response_decoded,
            by_alias=True,
            by_name=False,
        )
    else:
        response_decoded = decode_response_body(response)
        if response.status_code == 422:
            raise HttpValidationError.init_exception(
                response_decoded, response.status_code
            )
        else:
            raise HttpError.init_exception(response_decoded, response.status_code)
