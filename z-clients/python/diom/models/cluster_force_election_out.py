# this file is @generated

from ..internal.base_model import BaseModel


class ClusterForceElectionOut(BaseModel):
    previous_leader_id: str | None = None

    new_leader_id: str | None = None
