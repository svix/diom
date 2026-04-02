# this file is @generated

from pydantic import BaseModel


class KvDeleteOut(BaseModel):
    success: bool
    """Whether the operation succeeded or was a noop due to pre-conditions."""
