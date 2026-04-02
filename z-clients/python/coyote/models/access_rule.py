# this file is @generated
import typing as t

from pydantic import BaseModel

from .access_rule_effect import AccessRuleEffect


class AccessRule(BaseModel):
    effect: AccessRuleEffect

    resource: str

    actions: t.List[str]
