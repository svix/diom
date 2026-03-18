// this file is @generated

export interface ClusterRemoveNodeIn {
    nodeId: string;
}

export const ClusterRemoveNodeInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterRemoveNodeIn {
        return {
            nodeId: object['node_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterRemoveNodeIn): any {
        return {
            'node_id': self.nodeId,
        };
    }
}