# this file is @generated
import typing as t
from datetime import datetime

from pydantic import BaseModel


class ClusterForceSnapshotOut(BaseModel):
    snapshot_time: datetime
    """The wall-clock time at which the snapshot was initiated"""

    snapshot_log_index: int
    """The log index at which the snapshot was initiated"""

    snapshot_id: t.Optional[str] = None
    """If this is `null`, the snapshot is still building in the background"""
