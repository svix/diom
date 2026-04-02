# this file is @generated
import typing as t

from ..internal.base_model import BaseModel


class AdminAuthTokenUpdateIn(BaseModel):
    id: str

    name: t.Optional[str] = None

    expiry_ms: t.Optional[int] = None

    enabled: t.Optional[bool] = None
