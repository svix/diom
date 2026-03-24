// this file is @generated

export interface AuthTokenOut {
    id: string;
    name: string;
    created: Date;
    updated: Date;
    expiry?: Date | null;
    metadata: { [key: string]: string };
    ownerId: string;
    scopes: string[];
    /** Whether this token is currently enabled. */
    enabled: boolean;
}

export const AuthTokenOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenOut {
        return {
            id: object['id'],
            name: object['name'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
            expiry: object['expiry'] ? new Date(object['expiry']) : null,
            metadata: object['metadata'],
            ownerId: object['owner_id'],
            scopes: object['scopes'],
            enabled: object['enabled'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenOut): any {
        return {
            'id': self.id,
            'name': self.name,
            'created': self.created,
            'updated': self.updated,
            'expiry': self.expiry,
            'metadata': self.metadata,
            'owner_id': self.ownerId,
            'scopes': self.scopes,
            'enabled': self.enabled,
        };
    }
}