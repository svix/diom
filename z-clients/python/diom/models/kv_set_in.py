# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs

from .operation_behavior import OperationBehavior


class KvSetIn(BaseModel):
    namespace: str | None = None

    ttl: TimeDeltaMs | None = Field(alias="ttl_ms", default=None)
    """Time to live in milliseconds"""

    behavior: OperationBehavior | None = None

    version: int | None = None
    """If set, the write only succeeds when the stored version matches this value.
    Use the `version` field from a prior `get` response."""


class _KvSetIn(BaseModel):
    namespace: str | None = None

    key: str

    value: bytes

    ttl: TimeDeltaMs | None = Field(alias="ttl_ms", default=None)
    """Time to live in milliseconds"""

    behavior: OperationBehavior | None = None

    version: int | None = None
    """If set, the write only succeeds when the stored version matches this value.
    Use the `version` field from a prior `get` response."""
