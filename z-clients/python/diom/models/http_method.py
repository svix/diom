# this file is @generated
from enum import Enum


class HttpMethod(str, Enum):
    POST = "post"
    PUT = "put"
    PATCH = "patch"

    def __str__(self) -> str:
        return str(self.value)
