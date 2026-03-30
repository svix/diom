// this file is @generated

export interface AdminRoleGetIn {
    id: string;
}

export const AdminRoleGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleGetIn {
        return {
            id: object['id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleGetIn): any {
        return {
            'id': self.id,
        };
    }
}