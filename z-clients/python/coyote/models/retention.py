# this file is @generated
from pydantic import Field

from ..internal.base_model import BaseModel
from ..internal.types import TimeDeltaMs


class Retention(BaseModel):
    period: TimeDeltaMs | None = Field(alias="period_ms", default=None)
