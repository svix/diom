// this file is @generated

export interface AuthTokenUpdateIn {
    namespace?: string | null;
    id: string;
    name?: string | null;
    expiryMillis?: number | null;
    metadata?: { [key: string]: string } | null;
    scopes?: string[] | null;
    enabled?: boolean | null;
}

export const AuthTokenUpdateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenUpdateIn {
        return {
            namespace: object['namespace'],
            id: object['id'],
            name: object['name'],
            expiryMillis: object['expiry_millis'],
            metadata: object['metadata'],
            scopes: object['scopes'],
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenUpdateIn): any {
        return {
            'namespace': self.namespace,
            'id': self.id,
            'name': self.name,
            'expiry_millis': self.expiryMillis,
            'metadata': self.metadata,
            'scopes': self.scopes,
            'enabled': self.enabled,
        };
    }
}