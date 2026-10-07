// this file is @generated
import {
    type Consistency,
    ConsistencySerializer,
} from './consistency';

export interface MsgTopicListIn {
    namespace?: string | null;
    consistency?: Consistency;
    /** Limit the number of returned items */
    limit?: number;
    /** The iterator returned from a prior invocation */
    iterator?: string | null;
}

export const MsgTopicListInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicListIn {
        return {
            namespace: object['namespace'],
            consistency: object['consistency'] != null ? ConsistencySerializer._fromJsonObject(object['consistency']): undefined,
            limit: object['limit'],
            iterator: object['iterator'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicListIn): any {
        return {
            'namespace': self.namespace,
            'consistency': self.consistency != null ? ConsistencySerializer._toJsonObject(self.consistency) : undefined,
            'limit': self.limit,
            'iterator': self.iterator,
        };
    }
}