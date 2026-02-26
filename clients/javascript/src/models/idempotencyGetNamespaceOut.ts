// this file is @generated





export interface IdempotencyGetNamespaceOut {
    createdAt: Date;
maxStorageBytes?: number | null;
name: string;
updatedAt: Date;
}

export const IdempotencyGetNamespaceOutSerializer = {
    _fromJsonObject(object: any): IdempotencyGetNamespaceOut {
        return {
            createdAt: new Date(object['created_at']),
            maxStorageBytes: object['max_storage_bytes'],
            name: object['name'],
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: IdempotencyGetNamespaceOut): any {
        return {
            'created_at': self.createdAt,
            'max_storage_bytes': self.maxStorageBytes,
            'name': self.name,
            'updated_at': self.updatedAt,
            };
    }
}