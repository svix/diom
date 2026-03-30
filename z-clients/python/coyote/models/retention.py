# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class Retention(BaseModel):
    period_ms: t.Optional[int] = Field(default=None, alias="period_ms")

    size_bytes: t.Optional[int] = Field(default=None, alias="size_bytes")
