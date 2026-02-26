# this file is @generated
from pydantic import Field

from .common import BaseModel


class DlqIn(BaseModel):
    consumer_group: str = Field(alias="consumer_group")

    msg_id: int = Field(alias="msg_id")

    name: str
