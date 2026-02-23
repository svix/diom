// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface CacheGetGroupOut {
    createdAt: Date;
evictionPolicy: EvictionPolicy;
maxStorageBytes?: number | null;
name: string;
storageType: StorageType;
updatedAt: Date;
}

export const CacheGetGroupOutSerializer = {
    _fromJsonObject(object: any): CacheGetGroupOut {
        return {
            createdAt: new Date(object['created_at']),
            evictionPolicy: EvictionPolicySerializer._fromJsonObject(object['eviction_policy']),
            maxStorageBytes: object['max_storage_bytes'],
            name: object['name'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            updatedAt: new Date(object['updated_at']),
            };
    },

    _toJsonObject(self: CacheGetGroupOut): any {
        return {
            'created_at': self.createdAt,
            'eviction_policy': EvictionPolicySerializer._toJsonObject(self.evictionPolicy),
            'max_storage_bytes': self.maxStorageBytes,
            'name': self.name,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'updated_at': self.updatedAt,
            };
    }
}