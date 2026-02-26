# this file is @generated
import typing as t

from .common import BaseModel


class MsgIn(BaseModel):
    headers: t.Optional[t.Dict[str, str]] = None

    payload: t.List[int]
