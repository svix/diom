// this file is @generated
import {
    type MsgIn,
    MsgInSerializer,
} from './msgIn';





export interface PublishIn {
    msgs: MsgIn[];
topic: string;
}

export const PublishInSerializer = {
    _fromJsonObject(object: any): PublishIn {
        return {
            msgs: object['msgs'].map((item: MsgIn) => MsgInSerializer._fromJsonObject(item)),
            topic: object['topic'],
            };
    },

    _toJsonObject(self: PublishIn): any {
        return {
            'msgs': self.msgs.map((item) => MsgInSerializer._toJsonObject(item)),
            'topic': self.topic,
            };
    }
}