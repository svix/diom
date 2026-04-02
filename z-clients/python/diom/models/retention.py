# this file is @generated

from pydantic import BaseModel


class Retention(BaseModel):
    period_ms: int | None = None

    size_bytes: int | None = None
