// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';

export interface CacheConfigureNamespaceIn {
    name: string;
    evictionPolicy?: EvictionPolicy;
}

export const CacheConfigureNamespaceInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheConfigureNamespaceIn {
        return {
            name: object['name'],
            evictionPolicy: object['eviction_policy'] != null ? EvictionPolicySerializer._fromJsonObject(object['eviction_policy']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheConfigureNamespaceIn): any {
        return {
            'name': self.name,
            'eviction_policy': self.evictionPolicy != null ? EvictionPolicySerializer._toJsonObject(self.evictionPolicy) : undefined,
        };
    }
}