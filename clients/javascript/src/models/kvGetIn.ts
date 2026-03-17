// this file is @generated
import {
    type Consistency,
    ConsistencySerializer,
} from './consistency';

export interface KvGetIn {
    consistency?: Consistency;
}

export interface KvGetIn_ {
    key: string;
    consistency?: Consistency;
}

export const KvGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetIn_ {
        return {
            key: object['key'],
            consistency: object['consistency'] != null ? ConsistencySerializer._fromJsonObject(object['consistency']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetIn_): any {
        return {
            'key': self.key,
            'consistency': self.consistency != null ? ConsistencySerializer._toJsonObject(self.consistency) : undefined,
        };
    }
}