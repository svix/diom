# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class MsgStreamSeekIn(BaseModel):
    namespace: t.Optional[str] = None

    offset: t.Optional[int] = None

    position: t.Optional[str] = None


class _MsgStreamSeekIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str = Field(alias="consumer_group")

    offset: t.Optional[int] = None

    position: t.Optional[str] = None
