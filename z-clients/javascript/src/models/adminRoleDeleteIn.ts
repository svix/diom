// this file is @generated

export interface AdminRoleDeleteIn {
    id: string;
}

export const AdminRoleDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleDeleteIn {
        return {
            id: object['id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleDeleteIn): any {
        return {
            'id': self.id,
        };
    }
}