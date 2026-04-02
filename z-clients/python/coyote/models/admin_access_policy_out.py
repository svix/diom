# this file is @generated
import typing as t
from datetime import datetime

from pydantic import BaseModel

from .access_rule import AccessRule


class AdminAccessPolicyOut(BaseModel):
    id: str

    description: str

    rules: t.List[AccessRule]

    created: datetime

    updated: datetime
