// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';

export interface AuthTokenCreateNamespaceIn {
    name: string;
    storageType?: StorageType;
    maxStorageBytes?: number | null;
}

export const AuthTokenCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): AuthTokenCreateNamespaceIn {
        return {
            name: object['name'],
            storageType: object['storage_type'] != null ? StorageTypeSerializer._fromJsonObject(object['storage_type']): undefined,
            maxStorageBytes: object['max_storage_bytes'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: AuthTokenCreateNamespaceIn): any {
        return {
            'name': self.name,
            'storage_type': self.storageType != null ? StorageTypeSerializer._toJsonObject(self.storageType) : undefined,
            'max_storage_bytes': self.maxStorageBytes,
        };
    }
}