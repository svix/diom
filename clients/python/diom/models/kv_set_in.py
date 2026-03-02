# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .operation_behavior import OperationBehavior


class KvSetIn(BaseModel):
    key: str

    ttl: t.Optional[int] = None
    """Time to live in milliseconds"""

    behavior: t.Optional[OperationBehavior] = None

    value: bytes
