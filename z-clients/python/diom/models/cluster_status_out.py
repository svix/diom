# this file is @generated
import typing as t
from datetime import datetime

from ..internal.base_model import BaseModel

from .node_status_out import NodeStatusOut
from .server_state import ServerState


class ClusterStatusOut(BaseModel):
    cluster_id: str | None = None
    """The unique ID of this cluster.

    This value is populated on cluster initialization and will never change."""

    cluster_name: str | None = None
    """The name of this cluster (as defined in the config)

    This value is not replicated and should only be used for debugging."""

    this_node_id: str
    """The unique ID of the node servicing this request"""

    this_node_state: ServerState
    """The cluster state of the node servicing this request"""

    this_node_last_committed_timestamp: datetime
    """The timestamp of the last transaction committed on this node"""

    this_node_last_snapshot_id: str | None = None
    """The last snapshot taken on this node"""

    nodes: t.List[NodeStatusOut]
    """A list of all nodes known to be in the cluster"""
