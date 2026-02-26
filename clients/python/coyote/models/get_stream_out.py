# this file is @generated
import typing as t
from pydantic import Field
from datetime import datetime

from .common import BaseModel

from .storage_type import StorageType


class GetStreamOut(BaseModel):
    created_at: datetime = Field(alias="created_at")

    max_byte_size: t.Optional[int] = Field(default=None, alias="max_byte_size")

    name: str

    retention_period_seconds: t.Optional[int] = Field(
        default=None, alias="retention_period_seconds"
    )

    storage_type: StorageType = Field(alias="storage_type")

    updated_at: datetime = Field(alias="updated_at")
