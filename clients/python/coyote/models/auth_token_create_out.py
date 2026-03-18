# this file is @generated
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class AuthTokenCreateOut(BaseModel):
    id: str

    created_at: datetime = Field(alias="created_at")

    updated_at: datetime = Field(alias="updated_at")

    token: str
