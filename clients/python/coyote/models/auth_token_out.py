# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class AuthTokenOut(BaseModel):
    id: str

    name: str

    created_at: datetime = Field(alias="created_at")

    updated_at: datetime = Field(alias="updated_at")

    expiry: t.Optional[datetime] = None

    metadata: t.Dict[str, str]

    owner_id: str = Field(alias="owner_id")

    scopes: t.List[str]

    enabled: bool
    """Whether this token is currently enabled."""
