# this file is @generated
import typing as t

from .common import BaseModel


class Retention(BaseModel):
    bytes: t.Optional[int] = None

    millis: t.Optional[int] = None
