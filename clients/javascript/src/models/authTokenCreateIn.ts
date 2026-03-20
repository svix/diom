// this file is @generated

export interface AuthTokenCreateIn {
    namespace?: string | null;
    name: string;
    prefix?: string;
    suffix?: string | null;
    /** Milliseconds from now until the token expires. */
    expiryMillis?: number | null;
    metadata?: { [key: string]: string };
    ownerId: string;
    scopes?: string[];
    /** Whether the token is enabled. Defaults to `true`. */
    enabled?: boolean;
}

export const AuthTokenCreateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenCreateIn {
        return {
            namespace: object['namespace'],
            name: object['name'],
            prefix: object['prefix'],
            suffix: object['suffix'],
            expiryMillis: object['expiry_millis'],
            metadata: object['metadata'],
            ownerId: object['owner_id'],
            scopes: object['scopes'],
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenCreateIn): any {
        return {
            'namespace': self.namespace,
            'name': self.name,
            'prefix': self.prefix,
            'suffix': self.suffix,
            'expiry_millis': self.expiryMillis,
            'metadata': self.metadata,
            'owner_id': self.ownerId,
            'scopes': self.scopes,
            'enabled': self.enabled,
        };
    }
}