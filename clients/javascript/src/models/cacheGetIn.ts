// this file is @generated
import {
    type Consistency,
    ConsistencySerializer,
} from './consistency';

export interface CacheGetIn {
    consistency?: Consistency;
}

export interface CacheGetIn_ {
    key: string;
    consistency?: Consistency;
}

export const CacheGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheGetIn_ {
        return {
            key: object['key'],
            consistency: object['consistency'] != null ? ConsistencySerializer._fromJsonObject(object['consistency']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheGetIn_): any {
        return {
            'key': self.key,
            'consistency': self.consistency != null ? ConsistencySerializer._toJsonObject(self.consistency) : undefined,
        };
    }
}