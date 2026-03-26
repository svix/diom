# this file is @generated
from pydantic import Field
from datetime import datetime

from ..internal.base_model import BaseModel


class ClusterForceSnapshotOut(BaseModel):
    snapshot_time: datetime = Field(alias="snapshot_time")

    snapshot_log_index: int = Field(alias="snapshot_log_index")
