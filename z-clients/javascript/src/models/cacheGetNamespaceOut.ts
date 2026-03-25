// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';

export interface CacheGetNamespaceOut {
    name: string;
    maxStorageBytes?: number | null;
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
            'eviction_policy': EvictionPolicySerializer._toJsonObject(self.evictionPolicy),
            'created': self.created,
            'updated': self.updated,
        };
    }
}