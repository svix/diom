# this file is @generated

from ..internal.base_model import BaseModel
from ..internal.types import UnixTimestampMs


class ClusterForceSnapshotOut(BaseModel):
    snapshot_time: UnixTimestampMs
    """The wall-clock time at which the snapshot was initiated"""

    snapshot_log_index: int
    """The log index at which the snapshot was initiated"""

    snapshot_id: str | None = None
    """If this is `null`, the snapshot is still building in the background"""
