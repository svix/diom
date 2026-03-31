// this file is @generated

export interface AdminAccessPolicyDeleteOut {
    success: boolean;
}

export const AdminAccessPolicyDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyDeleteOut {
        return {
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyDeleteOut): any {
        return {
            'success': self.success,
        };
    }
}