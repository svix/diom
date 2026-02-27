# this file is @generated

from .common import BaseModel


class PublishOutMsg(BaseModel):
    offset: int

    partition: int
