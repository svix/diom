// this file is @generated

export interface ClusterForceNodeUpgradeIn {
    nodeId: string;
}

export const ClusterForceNodeUpgradeInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterForceNodeUpgradeIn {
        return {
            nodeId: object['node_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterForceNodeUpgradeIn): any {
        return {
            'node_id': self.nodeId,
        };
    }
}