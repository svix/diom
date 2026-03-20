// this file is @generated

export interface AdminAuthTokenDeleteIn {
    id: string;
}

export const AdminAuthTokenDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenDeleteIn {
        return {
            id: object['id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenDeleteIn): any {
        return {
            'id': self.id,
        };
    }
}