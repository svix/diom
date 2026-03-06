// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';

export interface CacheCreateNamespaceIn {
    name: string;
    storageType?: StorageType;
    maxStorageBytes?: number | null;
    evictionPolicy?: EvictionPolicy;
}

export const CacheCreateNamespaceInSerializer = {
    _fromJsonObject(object: any): CacheCreateNamespaceIn {
        return {
            name: object['name'],
            storageType: object['storage_type'] != null ? StorageTypeSerializer._fromJsonObject(object['storage_type']): undefined,
            maxStorageBytes: object['max_storage_bytes'],
            evictionPolicy: object['eviction_policy'] != null ? EvictionPolicySerializer._fromJsonObject(object['eviction_policy']): undefined,
        };
    },

    _toJsonObject(self: CacheCreateNamespaceIn): any {
        return {
            'name': self.name,
            'storage_type': self.storageType != null ? StorageTypeSerializer._toJsonObject(self.storageType) : undefined,
            'max_storage_bytes': self.maxStorageBytes,
            'eviction_policy': self.evictionPolicy != null ? EvictionPolicySerializer._toJsonObject(self.evictionPolicy) : undefined,
        };
    }
}