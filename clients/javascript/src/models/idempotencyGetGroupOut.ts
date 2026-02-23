// this file is @generated





export interface IdempotencyGetGroupOut {
    createdAt: Date;
maxStorageBytes?: number | null;
name: string;
updatedAt: Date;
}

export const IdempotencyGetGroupOutSerializer = {
    _fromJsonObject(object: any): IdempotencyGetGroupOut {
        return {
            createdAt: new Date(object['created_at']),
            maxStorageBytes: object['max_storage_bytes'],
            name: object['name'],
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: IdempotencyGetGroupOut): any {
        return {
            'created_at': self.createdAt,
            'max_storage_bytes': self.maxStorageBytes,
            'name': self.name,
            'updated_at': self.updatedAt,
            };
    }
}