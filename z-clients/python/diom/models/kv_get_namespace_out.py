# this file is @generated
from datetime import datetime

from pydantic import BaseModel


class KvGetNamespaceOut(BaseModel):
    name: str

    created: datetime

    updated: datetime
