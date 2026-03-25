// this file is @generated

export interface AdminAuthTokenDeleteOut {
    success: boolean;
}

export const AdminAuthTokenDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenDeleteOut {
        return {
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenDeleteOut): any {
        return {
            'success': self.success,
        };
    }
}