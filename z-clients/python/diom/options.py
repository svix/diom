from typing import List
from dataclasses import dataclass, field


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

    Defaults to [] (no automatic retries)
    """
    retry_schedule: List[float] = field(default_factory=lambda: [])

    """
    The maximum amount of time in seconds a request can take.

    Request methods will raise httpx.TimeoutException if this is exceeded.
    """
    timeout: float = 15.0

    proxy: str | None = None
