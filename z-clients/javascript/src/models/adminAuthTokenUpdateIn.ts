// this file is @generated

export interface AdminAuthTokenUpdateIn {
    id: string;
    name?: string | null;
    expiry?: Date | null;
    enabled?: boolean | null;
}

export const AdminAuthTokenUpdateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenUpdateIn {
        return {
            id: object['id'],
            name: object['name'],
            expiry: object['expiry_ms'] ? new Date(object['expiry_ms']) : null,
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenUpdateIn): any {
        return {
            'id': self.id,
            'name': self.name,
            'expiry_ms': self.expiry,
            'enabled': self.enabled,
        };
    }
}