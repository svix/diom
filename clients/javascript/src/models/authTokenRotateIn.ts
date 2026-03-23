// this file is @generated

export interface AuthTokenRotateIn {
    namespace?: string | null;
    id: string;
    prefix?: string;
    suffix?: string | null;
    /** Milliseconds from now until the old token expires. `None` means expire immediately. */
    expiryMs?: number | null;
}

export const AuthTokenRotateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenRotateIn {
        return {
            namespace: object['namespace'],
            id: object['id'],
            prefix: object['prefix'],
            suffix: object['suffix'],
            expiryMs: object['expiry_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenRotateIn): any {
        return {
            'namespace': self.namespace,
            'id': self.id,
            'prefix': self.prefix,
            'suffix': self.suffix,
            'expiry_ms': self.expiryMs,
        };
    }
}