// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';

export interface CacheConfigureNamespaceOut {
    name: string;
    evictionPolicy: EvictionPolicy;
    created: Date;
    updated: Date;
}

export const CacheConfigureNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheConfigureNamespaceOut {
        return {
            name: object['name'],
            evictionPolicy: EvictionPolicySerializer._fromJsonObject(object['eviction_policy']),
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheConfigureNamespaceOut): any {
        return {
            'name': self.name,
            'eviction_policy': EvictionPolicySerializer._toJsonObject(self.evictionPolicy),
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}