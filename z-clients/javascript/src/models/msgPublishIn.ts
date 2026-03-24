// this file is @generated
import {
    type MsgIn,
    MsgInSerializer,
} from './msgIn';

export interface MsgPublishIn {
    namespace?: string | null;
    msgs: MsgIn[];
}

export interface MsgPublishIn_ {
    namespace?: string | null;
    topic: string;
    msgs: MsgIn[];
}

export const MsgPublishInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgPublishIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            msgs: object['msgs'].map((item: MsgIn) => MsgInSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgPublishIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'msgs': self.msgs.map((item) => MsgInSerializer._toJsonObject(item)),
        };
    }
}