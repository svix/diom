// this file is @generated

export interface AdminRoleDeleteOut {
    success: boolean;
}

export const AdminRoleDeleteOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminRoleDeleteOut {
        return {
            success: object['success'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminRoleDeleteOut): any {
        return {
            'success': self.success,
        };
    }
}