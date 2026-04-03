# this file is @generated
import typing as t

from pydantic import BaseModel


class IdempotencyCompleted(BaseModel):
    response: bytes

    context: t.Dict[str, str] | None = None
