// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';

export interface RateLimitCreateNamespaceOut {
    name: string;
    maxStorageBytes?: number | null;
    storageType: StorageType;
    created: Date;
    updated: Date;
}

export const RateLimitCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCreateNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCreateNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'created': self.created,
            'updated': self.updated,
        };
    }
}