import ssl
from typing import Dict, List
from dataclasses import dataclass, field

import attr


DEFAULT_SERVER_URL = "http://localhost:8050"


@attr.s(auto_attribs=True)
class Client:
    """A class for keeping track of data related to the API

    Attributes:
        base_url: The base URL for the API, all requests are made to a relative path to this URL
        cookies: A dictionary of cookies to be sent with every request
        headers: A dictionary of headers to be sent with every request
        timeout: The maximum amount of a time in seconds a request can take. API functions will raise
            httpx.TimeoutException if this is exceeded.
        verify_ssl: Whether or not to verify the SSL certificate of the API server. This should be True in production,
            but can be set to False for testing purposes.
        raise_on_unexpected_status: Whether or not to raise an errors.UnexpectedStatus if the API returns a
            status code that was not documented in the source OpenAPI document.
        follow_redirects: Whether or not to follow redirects. Default value is False.
    """

    base_url: str
    cookies: Dict[str, str] = attr.ib(factory=dict, kw_only=True)
    headers: Dict[str, str] = attr.ib(factory=dict, kw_only=True)
    retry_schedule: List[float]
    timeout: float = attr.ib(5.0, kw_only=True)
    verify_ssl: str | bool | ssl.SSLContext = attr.ib(True, kw_only=True)
    raise_on_unexpected_status: bool = attr.ib(False, kw_only=True)
    follow_redirects: bool = attr.ib(False, kw_only=True)

    def get_headers(self) -> Dict[str, str]:
        """Get headers to be used in all endpoints"""
        return {**self.headers}

    def with_headers(self, headers: Dict[str, str]) -> "Client":
        """Get a new client matching this one with additional headers"""
        return attr.evolve(self, headers={**self.headers, **headers})

    def get_cookies(self) -> Dict[str, str]:
        return {**self.cookies}

    def with_cookies(self, cookies: Dict[str, str]) -> "Client":
        """Get a new client matching this one with additional cookies"""
        return attr.evolve(self, cookies={**self.cookies, **cookies})

    def get_timeout(self) -> float:
        return self.timeout

    def with_timeout(self, timeout: float) -> "Client":
        """Get a new client matching this one with a new timeout (in seconds)"""
        return attr.evolve(self, timeout=timeout)


@attr.s(auto_attribs=True)
class AuthenticatedClient(Client):
    """A Client which has been authenticated for use on secured endpoints"""

    token: str
    prefix: str = "Bearer"
    auth_header_name: str = "Authorization"
    proxy: str | None = attr.ib(default=None)

    def get_headers(self) -> Dict[str, str]:
        """Get headers to be used in authenticated endpoints"""
        auth_header_value = f"{self.prefix} {self.token}" if self.prefix else self.token
        return {self.auth_header_name: auth_header_value, **self.headers}


@dataclass
class DiomOptions:
    debug: bool = False
    server_url: str | None = None
    """
    The retry schedule, as seconds to wait after each failed request.

    The first entry is the time in seconds to wait between the first request
    failing and the first retry, and so on.
    Up to five retries are supported, passing a retry schedule with more than
    five entries will raise a `ValueError`.

    Defaults to [0.05, 0.1, 0.2]
    """
    retry_schedule: List[float] = field(default_factory=lambda: [0.05, 0.1, 0.2])

    """
    The maximum amount of time in seconds a request can take.

    Request methods will raise httpx.TimeoutException if this is exceeded.
    """
    timeout: float = 15.0

    proxy: str | None = None


class ClientBase:
    _client: AuthenticatedClient

    def __init__(
        self, auth_token: str, options: DiomOptions = DiomOptions()
    ) -> None:
        from .. import __version__

        if len(options.retry_schedule) > 5:
            raise ValueError("number of retries must not exceed 5")

        host = options.server_url or DEFAULT_SERVER_URL
        client = AuthenticatedClient(
            base_url=host,
            token=auth_token,
            headers={"user-agent": f"svix-libs/{__version__}/python"},
            verify_ssl=True,
            retry_schedule=options.retry_schedule,
            timeout=options.timeout,
            follow_redirects=False,
            raise_on_unexpected_status=True,
            proxy=options.proxy,
        )
        self._client = client
