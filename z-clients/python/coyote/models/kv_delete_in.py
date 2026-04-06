# this file is @generated

from ..internal.base_model import BaseModel


class KvDeleteIn(BaseModel):
    namespace: str | None = None

    version: int | None = None
    """If set, the delete only succeeds when the stored version matches this value.
    Use the `version` field from a prior `get` response."""


class _KvDeleteIn(BaseModel):
    namespace: str | None = None

    key: str

    version: int | None = None
    """If set, the delete only succeeds when the stored version matches this value.
    Use the `version` field from a prior `get` response."""
