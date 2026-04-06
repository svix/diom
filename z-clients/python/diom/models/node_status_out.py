# this file is @generated

from ..internal.base_model import BaseModel

from .server_state import ServerState


class NodeStatusOut(BaseModel):
    node_id: str
    """A unique ID representing this node.

    This will never change unless the node is erased and reset"""

    address: str
    """The advertised inter-server (cluster) address of this node."""

    state: ServerState
    """The last known state of this node"""

    last_committed_log_index: int | None = None
    """The index of the last log applied on this node"""

    last_committed_term: int | None = None
    """The raft term of the last committed leadership"""
