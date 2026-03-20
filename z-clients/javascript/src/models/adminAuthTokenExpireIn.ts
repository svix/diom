// this file is @generated

export interface AdminAuthTokenExpireIn {
    id: string;
    /** Milliseconds from now until the token expires. `None` means expire immediately. */
    expiryMs?: number | null;
}

export const AdminAuthTokenExpireInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AdminAuthTokenExpireIn {
        return {
            id: object['id'],
            expiryMs: object['expiry_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AdminAuthTokenExpireIn): any {
        return {
            'id': self.id,
            'expiry_ms': self.expiryMs,
        };
    }
}