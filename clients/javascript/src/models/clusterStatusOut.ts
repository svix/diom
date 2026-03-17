// this file is @generated
import {
    type NodeStatusOut,
    NodeStatusOutSerializer,
} from './nodeStatusOut';
import {
    type ServerState,
    ServerStateSerializer,
} from './serverState';

export interface ClusterStatusOut {
    /**
     * The unique ID of this cluster.pub(crate)
     * 
     * This value is populated on cluster initialization and will never change.
     */
    clusterId?: string | null;
    /**
     * The name of this cluster (as defined in the config)
     * 
     * This value is not replicated and should only be used for debugging.
     */
    clusterName?: string | null;
    /** The unique ID of the node servicing this request */
    thisNodeId: string;
    /** The cluster state of the node servicing this request */
    thisNodeState: ServerState;
    /** The timestamp of the last transaction committed on this node */
    thisNodeLastCommittedTimestamp: Date;
    /** A list of all nodes known to be in the cluster */
    nodes: NodeStatusOut[];
}

export const ClusterStatusOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterStatusOut {
        return {
            clusterId: object['cluster_id'],
            clusterName: object['cluster_name'],
            thisNodeId: object['this_node_id'],
            thisNodeState: ServerStateSerializer._fromJsonObject(object['this_node_state']),
            thisNodeLastCommittedTimestamp: new Date(object['this_node_last_committed_timestamp']),
            nodes: object['nodes'].map((item: NodeStatusOut) => NodeStatusOutSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterStatusOut): any {
        return {
            'cluster_id': self.clusterId,
            'cluster_name': self.clusterName,
            'this_node_id': self.thisNodeId,
            'this_node_state': ServerStateSerializer._toJsonObject(self.thisNodeState),
            'this_node_last_committed_timestamp': self.thisNodeLastCommittedTimestamp,
            'nodes': self.nodes.map((item) => NodeStatusOutSerializer._toJsonObject(item)),
        };
    }
}