// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface IdempotencyCreateNamespaceIn {
    name: string;
storageType?: StorageType;
maxStorageBytes?: number | null;
}

export const IdempotencyCreateNamespaceInSerializer = {
    _fromJsonObject(object: any): IdempotencyCreateNamespaceIn {
        return {
            name: object['name'],
            storageType: object['storage_type'] != null ? StorageTypeSerializer._fromJsonObject(object['storage_type']): undefined,
            maxStorageBytes: object['max_storage_bytes'],
            };
    },

    _toJsonObject(self: IdempotencyCreateNamespaceIn): any {
        return {
            'name': self.name,
            'storage_type': self.storageType != null ? StorageTypeSerializer._toJsonObject(self.storageType) : undefined,
            'max_storage_bytes': self.maxStorageBytes,
            };
    }
}