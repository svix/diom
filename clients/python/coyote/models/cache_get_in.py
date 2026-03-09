# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class CacheGetIn(BaseModel):
    linearizable: t.Optional[bool] = None
    """Whether or not the read should be linearizable

    If this is `true`, the read is guaranteed to see all previous operations, but will
    have to make at least one additional round-trip to the leader. If this is false, stale
    reads will be performed against the replica which receives this request."""


class _CacheGetIn(BaseModel):
    key: str

    linearizable: t.Optional[bool] = None
    """Whether or not the read should be linearizable

    If this is `true`, the read is guaranteed to see all previous operations, but will
    have to make at least one additional round-trip to the leader. If this is false, stale
    reads will be performed against the replica which receives this request."""
