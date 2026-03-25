// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';

export interface CacheCreateNamespaceIn {
    name: string;
    maxStorageBytes?: number | null;
    evictionPolicy?: EvictionPolicy;
}

export const CacheCreateNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheCreateNamespaceIn {
        return {
            name: object['name'],
            maxStorageBytes: object['max_storage_bytes'],
            evictionPolicy: object['eviction_policy'] != null ? EvictionPolicySerializer._fromJsonObject(object['eviction_policy']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheCreateNamespaceIn): any {
        return {
            'name': self.name,
            'max_storage_bytes': self.maxStorageBytes,
            'eviction_policy': self.evictionPolicy != null ? EvictionPolicySerializer._toJsonObject(self.evictionPolicy) : undefined,
        };
    }
}