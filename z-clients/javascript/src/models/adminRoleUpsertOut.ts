// this file is @generated

export interface AdminRoleUpsertOut {
    id: string;
    created: number;
    updated: number;
}

export const AdminRoleUpsertOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleUpsertOut {
        return {
            id: object['id'],
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleUpsertOut): any {
        return {
            'id': self.id,
            'created': self.created,
            'updated': self.updated,
        };
    }
}