# this file is @generated
from datetime import datetime

from pydantic import BaseModel


class AdminAccessPolicyUpsertOut(BaseModel):
    id: str

    created: datetime

    updated: datetime
