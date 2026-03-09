// this file is @generated
import {
    type MsgIn,
    MsgInSerializer,
} from './msgIn';

export interface MsgPublishIn {
    topic: string;
    msgs: MsgIn[];
}

export const MsgPublishInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgPublishIn {
        return {
            topic: object['topic'],
            msgs: object['msgs'].map((item: MsgIn) => MsgInSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgPublishIn): any {
        return {
            'topic': self.topic,
            'msgs': self.msgs.map((item) => MsgInSerializer._toJsonObject(item)),
        };
    }
}