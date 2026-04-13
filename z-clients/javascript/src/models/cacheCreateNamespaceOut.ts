// this file is @generated
import {
    type EvictionPolicy,
    EvictionPolicySerializer,
} from './evictionPolicy';

export interface CacheCreateNamespaceOut {
    name: string;
    evictionPolicy: EvictionPolicy;
    created: Date;
    updated: Date;
}

export const CacheCreateNamespaceOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheCreateNamespaceOut {
        return {
            name: object['name'],
            evictionPolicy: EvictionPolicySerializer._fromJsonObject(object['eviction_policy']),
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheCreateNamespaceOut): any {
        return {
            'name': self.name,
            'eviction_policy': EvictionPolicySerializer._toJsonObject(self.evictionPolicy),
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}