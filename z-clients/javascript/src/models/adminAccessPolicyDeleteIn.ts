// this file is @generated

export interface AdminAccessPolicyDeleteIn {
    id: string;
}

export const AdminAccessPolicyDeleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyDeleteIn {
        return {
            id: object['id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyDeleteIn): any {
        return {
            'id': self.id,
        };
    }
}