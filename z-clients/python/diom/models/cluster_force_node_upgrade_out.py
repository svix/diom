# this file is @generated

from ..internal.base_model import BaseModel

from .server_state import ServerState


class ClusterForceNodeUpgradeOut(BaseModel):
    initial_node_state: ServerState

    final_node_state: ServerState
