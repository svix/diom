// this file is @generated

export interface AuthTokenUpdateIn {
    namespace?: string | null;
    id: string;
    name?: string | null;
    expiryMs?: number | null;
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
            expiryMs: object['expiry_ms'],
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
            'expiry_ms': self.expiryMs,
            'metadata': self.metadata,
            'scopes': self.scopes,
            'enabled': self.enabled,
        };
    }
}