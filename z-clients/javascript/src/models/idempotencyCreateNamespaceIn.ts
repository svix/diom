// this file is @generated

export interface IdempotencyCreateNamespaceIn {
    name: string;
    maxStorageBytes?: number | null;
}

export const IdempotencyCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCreateNamespaceIn {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCreateNamespaceIn): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
        };
    }
}