// this file is @generated





export interface IdempotencyGetNamespaceOut {
    name: string;
maxStorageBytes?: number | null;
createdAt: Date;
updatedAt: Date;
}

export const IdempotencyGetNamespaceOutSerializer = {
    _fromJsonObject(object: any): IdempotencyGetNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            createdAt: new Date(object['created_at']),
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: IdempotencyGetNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'created_at': self.createdAt,
            'updated_at': self.updatedAt,
            };
    }
}