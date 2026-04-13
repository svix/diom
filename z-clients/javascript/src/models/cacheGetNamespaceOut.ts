// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';

export interface CacheGetNamespaceOut {
    name: string;
    evictionPolicy: EvictionPolicy;
    created: Date;
    updated: Date;
}

export const CacheGetNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetNamespaceOut {
        return {
            name: object['name'],
            evictionPolicy: EvictionPolicySerializer._fromJsonObject(object['eviction_policy']),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetNamespaceOut): any {
        return {
            'name': self.name,
            'eviction_policy': EvictionPolicySerializer._toJsonObject(self.evictionPolicy),
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}