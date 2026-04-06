# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class MsgIn(BaseModel):
    value: bytes

    headers: t.Dict[str, str] | None = None

    key: str | None = None
    """Optional partition key.

    Messages with the same key are routed to the same partition."""

    delay: TimeDeltaMs | None = Field(alias="delay_ms", default=None)
    """Optional delay in milliseconds.

    The message will not be delivered to queue consumers
    until the delay has elapsed from the time of publish."""
