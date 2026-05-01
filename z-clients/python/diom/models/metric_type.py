# this file is @generated
from enum import Enum


class MetricType(str, Enum):
    COUNTER = "counter"
    GAUGE = "gauge"

    def __str__(self) -> str:
        return str(self.value)
