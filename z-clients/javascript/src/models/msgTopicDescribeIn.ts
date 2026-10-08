// this file is @generated
import {
    type Consistency,
    ConsistencySerializer,
} from './consistency';

export interface MsgTopicDescribeIn {
    namespace?: string | null;
    consistency?: Consistency;
}

export interface MsgTopicDescribeIn_ {
    namespace?: string | null;
    topic: string;
    consistency?: Consistency;
}

export const MsgTopicDescribeInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgTopicDescribeIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            consistency: object['consistency'] != null ? ConsistencySerializer._fromJsonObject(object['consistency']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgTopicDescribeIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'consistency': self.consistency != null ? ConsistencySerializer._toJsonObject(self.consistency) : undefined,
        };
    }
}