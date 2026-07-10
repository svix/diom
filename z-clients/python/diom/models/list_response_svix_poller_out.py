# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .svix_poller_out import SvixPollerOut


class ListResponseSvixPollerOut(BaseModel):
    data: t.List[SvixPollerOut]

    iterator: str | None = None

    prev_iterator: str | None = None

    done: bool
