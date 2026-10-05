// this file is @generated
import {
    type ServerState,
    ServerStateSerializer,
} from './serverState';

export interface ClusterForceNodeUpgradeOut {
    initialNodeState: ServerState;
    finalNodeState: ServerState;
}

export const ClusterForceNodeUpgradeOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterForceNodeUpgradeOut {
        return {
            initialNodeState: ServerStateSerializer._fromJsonObject(object['initial_node_state']),
            finalNodeState: ServerStateSerializer._fromJsonObject(object['final_node_state']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterForceNodeUpgradeOut): any {
        return {
            'initial_node_state': ServerStateSerializer._toJsonObject(self.initialNodeState),
            'final_node_state': ServerStateSerializer._toJsonObject(self.finalNodeState),
        };
    }
}