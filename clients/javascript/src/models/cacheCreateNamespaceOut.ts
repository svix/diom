// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';
import {
    type StorageType,
    StorageTypeSerializer,
} from './storageType';





export interface CacheCreateNamespaceOut {
    name: string;
maxStorageBytes?: number | null;
storageType: StorageType;
evictionPolicy: EvictionPolicy;
created: Date;
updated: Date;
}

export const CacheCreateNamespaceOutSerializer = {
    _fromJsonObject(object: any): CacheCreateNamespaceOut {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            storageType: StorageTypeSerializer._fromJsonObject(object['storage_type']),
            evictionPolicy: EvictionPolicySerializer._fromJsonObject(object['eviction_policy']),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
            };
    },

    _toJsonObject(self: CacheCreateNamespaceOut): any {
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