# this file is @generated
import typing as t

from pydantic import BaseModel


class Retention(BaseModel):
    period_ms: t.Optional[int] = None

    size_bytes: t.Optional[int] = None
