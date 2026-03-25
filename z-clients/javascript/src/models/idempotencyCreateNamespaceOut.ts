// this file is @generated

export interface IdempotencyCreateNamespaceOut {
    name: string;
    maxStorageBytes?: number | null;
    created: Date;
    updated: Date;
}

export const IdempotencyCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCreateNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCreateNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'created': self.created,
            'updated': self.updated,
        };
    }
}