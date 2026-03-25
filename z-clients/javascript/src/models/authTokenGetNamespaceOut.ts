// this file is @generated

export interface AuthTokenGetNamespaceOut {
    name: string;
    maxStorageBytes?: number | null;
    created: Date;
    updated: Date;
}

export const AuthTokenGetNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenGetNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenGetNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'created': self.created,
            'updated': self.updated,
        };
    }
}