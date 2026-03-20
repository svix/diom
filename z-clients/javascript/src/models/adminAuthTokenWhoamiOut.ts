// this file is @generated

export interface AdminAuthTokenWhoamiOut {
    role: string;
}

export const AdminAuthTokenWhoamiOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenWhoamiOut {
        return {
            role: object['role'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenWhoamiOut): any {
        return {
            'role': self.role,
        };
    }
}