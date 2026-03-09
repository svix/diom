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
    created: Date;
    updated: Date;
}

export const CacheGetNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            evictionPolicy: EvictionPolicySerializer._fromJsonObject(object['eviction_policy']),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetNamespaceOut): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'storage_type': StorageTypeSerializer._toJsonObject(self.storageType),
            'eviction_policy': EvictionPolicySerializer._toJsonObject(self.evictionPolicy),
            'created': self.created,
            'updated': self.updated,
        };
    }
}