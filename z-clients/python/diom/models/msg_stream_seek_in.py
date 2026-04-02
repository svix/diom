# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .seek_position import SeekPosition


class MsgStreamSeekIn(BaseModel):
    namespace: t.Optional[str] = None

    offset: t.Optional[int] = None

    position: t.Optional[SeekPosition] = None


class _MsgStreamSeekIn(BaseModel):
    namespace: t.Optional[str] = None

    topic: str

    consumer_group: str

    offset: t.Optional[int] = None

    position: t.Optional[SeekPosition] = None
