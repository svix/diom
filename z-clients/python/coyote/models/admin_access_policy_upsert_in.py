# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .access_rule import AccessRule


class AdminAccessPolicyUpsertIn(BaseModel):
    id: str

    description: str

    rules: t.Optional[t.List[AccessRule]] = None
