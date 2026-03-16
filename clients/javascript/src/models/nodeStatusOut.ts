// this file is @generated
import {
    type ServerState,
    ServerStateSerializer,
} from './serverState';

export interface NodeStatusOut {
    /**
* A unique ID representing this node.
* 
* This will never change unless the node is erased and reset
*/
    nodeId: string;
    /** The advertised inter-server (cluster) address of this node. */
    address: string;
    /** The last known state of this node */
    state: ServerState;
    /** The index of the last log applied on this node */
    lastCommittedLogIndex?: number | null;
    /** The raft term of the last committed leadership */
    lastCommittedTerm?: number | null;
}

export const NodeStatusOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): NodeStatusOut {
        return {
            nodeId: object['node_id'],
            address: object['address'],
            state: ServerStateSerializer._fromJsonObject(object['state']),
            lastCommittedLogIndex: object['last_committed_log_index'],
            lastCommittedTerm: object['last_committed_term'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: NodeStatusOut): any {
        return {
            'node_id': self.nodeId,
            'address': self.address,
            'state': ServerStateSerializer._toJsonObject(self.state),
            'last_committed_log_index': self.lastCommittedLogIndex,
            'last_committed_term': self.lastCommittedTerm,
        };
    }
}