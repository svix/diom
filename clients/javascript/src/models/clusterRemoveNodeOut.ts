// this file is @generated

export interface ClusterRemoveNodeOut {
    nodeId: string;
}

export const ClusterRemoveNodeOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterRemoveNodeOut {
        return {
            nodeId: object['node_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterRemoveNodeOut): any {
        return {
            'node_id': self.nodeId,
        };
    }
}