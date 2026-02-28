// this file is @generated
import {
    type MsgIn,
    MsgInSerializer,
} from './msgIn';





export interface MsgPublishIn {
    msgs: MsgIn[];
topic: string;
}

export const MsgPublishInSerializer = {
    _fromJsonObject(object: any): MsgPublishIn {
        return {
            msgs: object['msgs'].map((item: MsgIn) => MsgInSerializer._fromJsonObject(item)),
            topic: object['topic'],
            };
    },

    _toJsonObject(self: MsgPublishIn): any {
        return {
            'msgs': self.msgs.map((item) => MsgInSerializer._toJsonObject(item)),
            'topic': self.topic,
            };
    }
}