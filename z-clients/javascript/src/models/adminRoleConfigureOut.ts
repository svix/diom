// this file is @generated

export interface AdminRoleConfigureOut {
    id: string;
    created: Date;
    updated: Date;
}

export const AdminRoleConfigureOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleConfigureOut {
        return {
            id: object['id'],
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleConfigureOut): any {
        return {
            'id': self.id,
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}