// this file is @generated

export interface AdminAuthTokenOut {
    id: string;
    name: string;
    created: number;
    updated: number;
    expiry?: number | null;
    role: string;
    /** Whether this token is currently enabled. */
    enabled: boolean;
}

export const AdminAuthTokenOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenOut {
        return {
            id: object['id'],
            name: object['name'],
            created: object['created'],
            updated: object['updated'],
            expiry: object['expiry'],
            role: object['role'],
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenOut): any {
        return {
            'id': self.id,
            'name': self.name,
            'created': self.created,
            'updated': self.updated,
            'expiry': self.expiry,
            'role': self.role,
            'enabled': self.enabled,
        };
    }
}