// this file is @generated

export interface AdminAccessPolicyGetIn {
    id: string;
}

export const AdminAccessPolicyGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAccessPolicyGetIn {
        return {
            id: object['id'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAccessPolicyGetIn): any {
        return {
            'id': self.id,
        };
    }
}