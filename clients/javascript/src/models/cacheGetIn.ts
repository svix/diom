// this file is @generated
import {
    type Consistency,
    ConsistencySerializer,
} from './consistency';

export interface CacheGetIn {
    namespace?: string | null;
    consistency?: Consistency;
}

export interface CacheGetIn_ {
    namespace?: string | null;
    key: string;
    consistency?: Consistency;
}

export const CacheGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            consistency: object['consistency'] != null ? ConsistencySerializer._fromJsonObject(object['consistency']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'consistency': self.consistency != null ? ConsistencySerializer._toJsonObject(self.consistency) : undefined,
        };
    }
}