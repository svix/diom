# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .access_rule_effect import AccessRuleEffect
from .resource_pattern import ResourcePattern


class AccessRule(BaseModel):
    effect: AccessRuleEffect

    resource: ResourcePattern

    actions: t.List[str]
