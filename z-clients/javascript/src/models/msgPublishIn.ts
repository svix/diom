// this file is @generated
import {
    type MsgIn,
    MsgInSerializer,
} from './msgIn';

export interface MsgPublishIn {
    namespace?: string | null;
    msgs: MsgIn[];
    idempotencyKey?: string | null;
}

export interface MsgPublishIn_ {
    namespace?: string | null;
    topic: string;
    msgs: MsgIn[];
    idempotencyKey?: string | null;
}

export const MsgPublishInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgPublishIn_ {
        return {
            namespace: object['namespace'],
            topic: object['topic'],
            msgs: object['msgs'].map((item: MsgIn) => MsgInSerializer._fromJsonObject(item)),
            idempotencyKey: object['idempotency_key'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgPublishIn_): any {
        return {
            'namespace': self.namespace,
            'topic': self.topic,
            'msgs': self.msgs.map((item) => MsgInSerializer._toJsonObject(item)),
            'idempotency_key': self.idempotencyKey,
        };
    }
}