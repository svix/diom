// this file is @generated

export interface ClusterForceSnapshotOut {
    snapshotTime: Date;
    snapshotLogIndex: number;
}

export const ClusterForceSnapshotOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterForceSnapshotOut {
        return {
            snapshotTime: new Date(object['snapshot_time']),
            snapshotLogIndex: object['snapshot_log_index'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterForceSnapshotOut): any {
        return {
            'snapshot_time': self.snapshotTime,
            'snapshot_log_index': self.snapshotLogIndex,
        };
    }
}