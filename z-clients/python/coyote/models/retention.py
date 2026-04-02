# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class Retention(BaseModel):
    period_ms: t.Optional[int] = None

    size_bytes: t.Optional[int] = None
