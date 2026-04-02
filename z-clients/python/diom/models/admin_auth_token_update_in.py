# this file is @generated

from pydantic import BaseModel


class AdminAuthTokenUpdateIn(BaseModel):
    id: str

    name: str | None = None

    expiry_ms: int | None = None

    enabled: bool | None = None
