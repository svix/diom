// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface KvGetNamespaceOut {
    name: string;
maxStorageBytes?: number | null;
storageType: StorageType;
createdAt: Date;
updatedAt: Date;
}

export const KvGetNamespaceOutSerializer = {
    _fromJsonObject(object: any): KvGetNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            createdAt: new Date(object['created_at']),
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: KvGetNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'created_at': self.createdAt,
            'updated_at': self.updatedAt,
            };
    }
}