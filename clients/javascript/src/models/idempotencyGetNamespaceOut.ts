// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface IdempotencyGetNamespaceOut {
    name: string;
maxStorageBytes?: number | null;
storageType: StorageType;
created: Date;
updated: Date;
}

export const IdempotencyGetNamespaceOutSerializer = {
    _fromJsonObject(object: any): IdempotencyGetNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
            };
    },

    _toJsonObject(self: IdempotencyGetNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'created': self.created,
            'updated': self.updated,
            };
    }
}