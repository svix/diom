# this file is @generated
from pydantic import Field

from .common import BaseModel


class RateLimiterFixedWindowConfig(BaseModel):
    max_requests: int = Field(alias="max_requests")
    """Maximum number of requests allowed within the window"""

    window_size: int = Field(alias="window_size")
    """Window size in seconds"""
