# this file is @generated

from ..internal.base_model import BaseModel


class SinkDeleteOut(BaseModel):
    topic: str

    consumer_group: str

    success: bool
