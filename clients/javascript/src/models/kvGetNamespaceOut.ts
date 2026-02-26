// this file is @generated
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface KvGetNamespaceOut {
    createdAt: Date;
maxStorageBytes?: number | null;
name: string;
storageType: StorageType;
updatedAt: Date;
}

export const KvGetNamespaceOutSerializer = {
    _fromJsonObject(object: any): KvGetNamespaceOut {
        return {
            createdAt: new Date(object['created_at']),
            maxStorageBytes: object['max_storage_bytes'],
            name: object['name'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: KvGetNamespaceOut): any {
        return {
            'created_at': self.createdAt,
            'max_storage_bytes': self.maxStorageBytes,
            'name': self.name,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'updated_at': self.updatedAt,
            };
    }
}