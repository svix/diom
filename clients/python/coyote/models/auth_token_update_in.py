# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AuthTokenUpdateIn(BaseModel):
    namespace: t.Optional[str] = None

    id: str

    name: t.Optional[str] = None

    expiry_millis: t.Optional[int] = Field(default=None, alias="expiry_millis")

    metadata: t.Optional[t.Dict[str, str]] = None

    scopes: t.Optional[t.List[str]] = None

    enabled: t.Optional[bool] = None
