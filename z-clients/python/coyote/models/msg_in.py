# this file is @generated
import typing as t

from pydantic import BaseModel


class MsgIn(BaseModel):
    value: bytes

    headers: t.Optional[t.Dict[str, str]] = None

    key: t.Optional[str] = None
    """Optional partition key.

    Messages with the same key are routed to the same partition."""

    delay_ms: t.Optional[int] = None
    """Optional delay in milliseconds.

    The message will not be delivered to queue consumers
    until the delay has elapsed from the time of publish."""
