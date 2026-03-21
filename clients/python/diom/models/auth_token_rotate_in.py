# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AuthTokenRotateIn(BaseModel):
    namespace: t.Optional[str] = None

    id: str

    prefix: t.Optional[str] = None

    suffix: t.Optional[str] = None

    expiry_millis: t.Optional[int] = Field(default=None, alias="expiry_millis")
    """Milliseconds from now until the old token expires. `None` means expire immediately."""
