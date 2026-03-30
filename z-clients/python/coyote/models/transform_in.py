# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class TransformIn(BaseModel):
    input: str
    """JSON-encoded payload passed to the script as `input`."""

    script: str
    """JavaScript source. Must define a `handler(input)` function."""

    max_duration_ms: t.Optional[int] = Field(default=None, alias="max_duration_ms")
    """How long to let the script run before being killed."""
