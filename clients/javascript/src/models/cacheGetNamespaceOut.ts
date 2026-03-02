// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface CacheGetNamespaceOut {
    name: string;
maxStorageBytes?: number | null;
storageType: StorageType;
evictionPolicy: EvictionPolicy;
createdAt: Date;
updatedAt: Date;
}

export const CacheGetNamespaceOutSerializer = {
    _fromJsonObject(object: any): CacheGetNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            evictionPolicy: EvictionPolicySerializer._fromJsonObject(object['eviction_policy']),
            createdAt: new Date(object['created_at']),
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: CacheGetNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'eviction_policy': EvictionPolicySerializer._toJsonObject(self.evictionPolicy),
            'created_at': self.createdAt,
            'updated_at': self.updatedAt,
            };
    }
}