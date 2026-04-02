# this file is @generated
import typing as t

from pydantic import BaseModel

from .access_rule import AccessRule


class AdminRoleUpsertIn(BaseModel):
    id: str

    description: str

    rules: t.List[AccessRule] | None = None

    policies: t.List[str] | None = None

    context: t.Dict[str, str] | None = None
