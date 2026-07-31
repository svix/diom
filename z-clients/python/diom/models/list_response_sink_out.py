# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .sink_out import SinkOut


class ListResponseSinkOut(BaseModel):
    data: t.List[SinkOut]

    iterator: str | None = None

    prev_iterator: str | None = None

    done: bool
