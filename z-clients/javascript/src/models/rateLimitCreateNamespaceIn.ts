// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';

export interface RateLimitCreateNamespaceIn {
    name: string;
    storageType?: StorageType;
    maxStorageBytes?: number | null;
}

export const RateLimitCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCreateNamespaceIn {
        return {
            name: object['name'],
            storageType: object['storage_type'] != null ? StorageTypeSerializer._fromJsonObject(object['storage_type']): undefined,
            maxStorageBytes: object['max_storage_bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCreateNamespaceIn): any {
        return {
            'name': self.name,
            'storage_type': self.storageType != null ? StorageTypeSerializer._toJsonObject(self.storageType) : undefined,
            'max_storage_bytes': self.maxStorageBytes,
        };
    }
}