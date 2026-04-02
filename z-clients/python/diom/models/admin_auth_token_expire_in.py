# this file is @generated

from pydantic import BaseModel


class AdminAuthTokenExpireIn(BaseModel):
    id: str

    expiry_ms: int | None = None
    """Milliseconds from now until the token expires. `None` means expire immediately."""
