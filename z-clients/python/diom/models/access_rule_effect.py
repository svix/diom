# this file is @generated
from enum import Enum


class AccessRuleEffect(str, Enum):
    ALLOW = "allow"
    DENY = "deny"

    def __str__(self) -> str:
        return str(self.value)
