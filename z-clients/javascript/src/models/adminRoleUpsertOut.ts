// this file is @generated

export interface AdminRoleUpsertOut {
    id: string;
    created: Date;
    updated: Date;
}

export const AdminRoleUpsertOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleUpsertOut {
        return {
            id: object['id'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleUpsertOut): any {
        return {
            'id': self.id,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}