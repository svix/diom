# this file is @generated
import typing as t

from ..internal.base_model import BaseModel

from .admin_auth_token_out import AdminAuthTokenOut


class ListResponseAdminAuthTokenOut(BaseModel):
    data: t.List[AdminAuthTokenOut]

    iterator: str | None = None

    prev_iterator: str | None = None

    done: bool
