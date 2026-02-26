# this file is @generated
import typing as t
from pydantic import Field

from .common import BaseModel


class AppendToStreamOut(BaseModel):
    msg_ids: t.List[int] = Field(alias="msg_ids")
