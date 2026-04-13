// this file is @generated

export interface ClusterForceSnapshotOut {
    /** The wall-clock time at which the snapshot was initiated */
    snapshotTime: Date;
    /** The log index at which the snapshot was initiated */
    snapshotLogIndex: number;
    /** If this is `null`, the snapshot is still building in the background */
    snapshotId?: string | null;
}

export const ClusterForceSnapshotOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterForceSnapshotOut {
        return {
            snapshotTime: new Date(object['snapshot_time']),
            snapshotLogIndex: object['snapshot_log_index'],
            snapshotId: object['snapshot_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterForceSnapshotOut): any {
        return {
            'snapshot_time': self.snapshotTime.getTime(),
            'snapshot_log_index': self.snapshotLogIndex,
            'snapshot_id': self.snapshotId,
        };
    }
}