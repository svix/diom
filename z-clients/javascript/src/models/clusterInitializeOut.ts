// this file is @generated

export interface ClusterInitializeOut {
    clusterId: string;
}

export const ClusterInitializeOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterInitializeOut {
        return {
            clusterId: object['cluster_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterInitializeOut): any {
        return {
            'cluster_id': self.clusterId,
        };
    }
}