# this file is @generated

from ..internal.base_model import BaseModel


class SvixPollerDeleteIn(BaseModel):
    namespace: str | None = None

    topic: str

    poller_id: str
