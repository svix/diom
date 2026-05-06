// this file is @generated

export interface ClusterForceElectionOut {
    previousLeaderId?: string | null;
    newLeaderId?: string | null;
}

export const ClusterForceElectionOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ClusterForceElectionOut {
        return {
            previousLeaderId: object['previous_leader_id'],
            newLeaderId: object['new_leader_id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ClusterForceElectionOut): any {
        return {
            'previous_leader_id': self.previousLeaderId,
            'new_leader_id': self.newLeaderId,
        };
    }
}