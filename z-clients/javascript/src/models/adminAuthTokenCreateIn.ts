// this file is @generated

export interface AdminAuthTokenCreateIn {
    name: string;
    role: string;
    /** Milliseconds from now until the token expires. */
    expiry?: Date | null;
    /** Whether the token is enabled. Defaults to `true`. */
    enabled?: boolean;
}

export const AdminAuthTokenCreateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenCreateIn {
        return {
            name: object['name'],
            role: object['role'],
            expiryMs: object['expiry_ms'] ? new Date(object['expiry_ms']) : null,
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenCreateIn): any {
        return {
            'name': self.name,
            'role': self.role,
            'expiry_ms': self.expiryMs,
            'enabled': self.enabled,
        };
    }
}