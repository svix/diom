// this file is @generated

export interface AuthTokenCreateNamespaceIn {
    name: string;
    maxStorageBytes?: number | null;
}

export const AuthTokenCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenCreateNamespaceIn {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenCreateNamespaceIn): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
        };
    }
}