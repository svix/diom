# this file is @generated
from datetime import datetime

from pydantic import BaseModel

from .retention import Retention


class MsgNamespaceGetOut(BaseModel):
    name: str

    retention: Retention

    created: datetime

    updated: datetime
