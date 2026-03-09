// this file is @generated
import {
    type Consistency,
    ConsistencySerializer,
} from './consistency';

export interface KvGetIn {
    key: string;
    consistency?: Consistency;
}

export const KvGetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvGetIn {
        return {
            key: object['key'],
            consistency: object['consistency'] != null ? ConsistencySerializer._fromJsonObject(object['consistency']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvGetIn): any {
        return {
            'key': self.key,
            'consistency': self.consistency != null ? ConsistencySerializer._toJsonObject(self.consistency) : undefined,
        };
    }
}