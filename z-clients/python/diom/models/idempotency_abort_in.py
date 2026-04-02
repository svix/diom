# this file is @generated

from pydantic import BaseModel


class IdempotencyAbortIn(BaseModel):
    namespace: str | None = None


class _IdempotencyAbortIn(BaseModel):
    namespace: str | None = None

    key: str
