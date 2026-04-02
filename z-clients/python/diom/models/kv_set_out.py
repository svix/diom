# this file is @generated

from pydantic import BaseModel


class KvSetOut(BaseModel):
    success: bool
    """Whether the operation succeeded or was a noop due to pre-conditions."""

    version: int
