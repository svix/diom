# this file is @generated

from ..internal.base_model import BaseModel

from .consistency import Consistency


class KvGetIn(BaseModel):
    namespace: str | None = None

    consistency: Consistency | None = None

    use_postgres: bool | None = None
    """If true, fetch from postgres instead of fjall (for benchmarking)."""


class _KvGetIn(BaseModel):
    namespace: str | None = None

    key: str

    consistency: Consistency | None = None

    use_postgres: bool | None = None
    """If true, fetch from postgres instead of fjall (for benchmarking)."""
