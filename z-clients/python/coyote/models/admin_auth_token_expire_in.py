# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class AdminAuthTokenExpireIn(BaseModel):
    id: str

    expiry_ms: t.Optional[int] = None
    """Milliseconds from now until the token expires. `None` means expire immediately."""
