# this file is @generated

from ..internal.base_model import BaseModel


class SvixPollerDeleteOut(BaseModel):
    topic: str

    poller_id: str

    success: bool
