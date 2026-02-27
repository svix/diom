// this file is @generated
import {
    type MsgIn2,
    MsgIn2Serializer,
} from './msgIn2';





export interface PublishIn {
    msgs: MsgIn2[];
name: string;
topic: string;
}

export const PublishInSerializer = {
    _fromJsonObject(object: any): PublishIn {
        return {
            msgs: object['msgs'].map((item: MsgIn2) => MsgIn2Serializer._fromJsonObject(item)),
            name: object['name'],
            topic: object['topic'],
            };
    },

    _toJsonObject(self: PublishIn): any {
        return {
            'msgs': self.msgs.map((item) => MsgIn2Serializer._toJsonObject(item)),
            'name': self.name,
            'topic': self.topic,
            };
    }
}