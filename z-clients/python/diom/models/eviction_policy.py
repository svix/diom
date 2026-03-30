# this file is @generated
from enum import Enum


class EvictionPolicy(str, Enum):
    NO_EVICTION = "NoEviction"

    def __str__(self) -> str:
        return str(self.value)
