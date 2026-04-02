# this file is @generated
import typing as t

from pydantic import BaseModel


class IdempotencyAbortIn(BaseModel):
    namespace: t.Optional[str] = None


class _IdempotencyAbortIn(BaseModel):
    namespace: t.Optional[str] = None

    key: str
