# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .access_rule import AccessRule


class AdminAccessPolicyConfigureIn(BaseModel):
    id: str

    description: str

    rules: t.List[AccessRule] | None = None
