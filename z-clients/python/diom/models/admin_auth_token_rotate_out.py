# this file is @generated

from ..internal.base_model import BaseModel


class AdminAuthTokenRotateOut(BaseModel):
    id: str

    token: str

    created: int

    updated: int
