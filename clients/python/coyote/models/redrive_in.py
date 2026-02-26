# this file is @generated
from pydantic import Field

from .common import BaseModel


class RedriveIn(BaseModel):
    consumer_group: str = Field(alias="consumer_group")

    name: str
