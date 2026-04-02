# this file is @generated
from datetime import datetime

from pydantic import BaseModel


class IdempotencyGetNamespaceOut(BaseModel):
    name: str

    created: datetime

    updated: datetime
