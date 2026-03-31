// this file is @generated

export interface AdminAccessPolicyUpsertOut {
    id: string;
    created: Date;
    updated: Date;
}

export const AdminAccessPolicyUpsertOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyUpsertOut {
        return {
            id: object['id'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
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