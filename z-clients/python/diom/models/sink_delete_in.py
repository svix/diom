# this file is @generated

from ..internal.base_model import BaseModel


class SinkDeleteIn(BaseModel):
    namespace: str | None = None

    topic: str

    consumer_group: str
