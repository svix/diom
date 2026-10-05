// this file is @generated

export interface ClusterForceNodeDowngradeIn {
    nodeId: string;
}

export const ClusterForceNodeDowngradeInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterForceNodeDowngradeIn {
        return {
            nodeId: object['node_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterForceNodeDowngradeIn): any {
        return {
            'node_id': self.nodeId,
        };
    }
}