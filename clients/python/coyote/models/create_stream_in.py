# this file is @generated
import typing as t
from pydantic import Field

from .common import BaseModel


class CreateStreamIn(BaseModel):
    max_byte_size: t.Optional[int] = Field(default=None, alias="max_byte_size")
    """How many bytes in total the stream will retain before dropping data."""

    name: str

    retention_period_seconds: t.Optional[int] = Field(
        default=None, alias="retention_period_seconds"
    )
    """How long messages are retained in the stream before being permanently nuked."""
