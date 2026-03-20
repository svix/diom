// this file is @generated

export interface AuthTokenExpireIn {
    namespace?: string | null;
    id: string;
    /** Milliseconds from now until the token expires. `None` means expire immediately. */
    expiryMillis?: number | null;
}

export const AuthTokenExpireInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenExpireIn {
        return {
            namespace: object['namespace'],
            id: object['id'],
            expiryMillis: object['expiry_millis'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenExpireIn): any {
        return {
            'namespace': self.namespace,
            'id': self.id,
            'expiry_millis': self.expiryMillis,
        };
    }
}