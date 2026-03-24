# this file is @generated
import typing as t
from pydantic import Field

from ..internal.base_model import BaseModel


class AuthTokenListIn(BaseModel):
    namespace: t.Optional[str] = None

    owner_id: str = Field(alias="owner_id")
