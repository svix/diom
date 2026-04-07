// this file is @generated

export interface AdminAccessPolicyUpsertOut {
    id: string;
    created: number;
    updated: number;
}

export const AdminAccessPolicyUpsertOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyUpsertOut {
        return {
            id: object['id'],
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyUpsertOut): any {
        return {
            'id': self.id,
            'created': self.created,
            'updated': self.updated,
        };
    }
}