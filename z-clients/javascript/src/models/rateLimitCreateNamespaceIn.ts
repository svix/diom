// this file is @generated

export interface RateLimitCreateNamespaceIn {
    name: string;
    maxStorageBytes?: number | null;
}

export const RateLimitCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCreateNamespaceIn {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCreateNamespaceIn): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
        };
    }
}