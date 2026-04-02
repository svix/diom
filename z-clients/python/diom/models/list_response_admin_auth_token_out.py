# this file is @generated
import typing as t

from pydantic import BaseModel

from .admin_auth_token_out import AdminAuthTokenOut


class ListResponseAdminAuthTokenOut(BaseModel):
    data: t.List[AdminAuthTokenOut]

    iterator: t.Optional[str] = None

    prev_iterator: t.Optional[str] = None

    done: bool
