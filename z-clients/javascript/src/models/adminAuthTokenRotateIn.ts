// this file is @generated

export interface AdminAuthTokenRotateIn {
    id: string;
}

export const AdminAuthTokenRotateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenRotateIn {
        return {
            id: object['id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenRotateIn): any {
        return {
            'id': self.id,
        };
    }
}