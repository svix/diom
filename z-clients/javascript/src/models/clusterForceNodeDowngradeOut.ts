// this file is @generated
import {
    type ServerState,
    ServerStateSerializer,
} from './serverState';

export interface ClusterForceNodeDowngradeOut {
    initialNodeState: ServerState;
    finalNodeState: ServerState;
}

export const ClusterForceNodeDowngradeOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterForceNodeDowngradeOut {
        return {
            initialNodeState: ServerStateSerializer._fromJsonObject(object['initial_node_state']),
            finalNodeState: ServerStateSerializer._fromJsonObject(object['final_node_state']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterForceNodeDowngradeOut): any {
        return {
            'initial_node_state': ServerStateSerializer._toJsonObject(self.initialNodeState),
            'final_node_state': ServerStateSerializer._toJsonObject(self.finalNodeState),
        };
    }
}