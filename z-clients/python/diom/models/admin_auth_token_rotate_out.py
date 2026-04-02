# this file is @generated
from datetime import datetime

from pydantic import BaseModel


class AdminAuthTokenRotateOut(BaseModel):
    id: str

    token: str

    created: datetime

    updated: datetime
