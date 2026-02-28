// this file is @generated
import {
    type MsgPublishOutMsg,
    MsgPublishOutMsgSerializer,
} from './msgPublishOutMsg';





export interface MsgPublishOut {
    msgs: MsgPublishOutMsg[];
}

export const MsgPublishOutSerializer = {
    _fromJsonObject(object: any): MsgPublishOut {
        return {
            msgs: object['msgs'].map((item: MsgPublishOutMsg) => MsgPublishOutMsgSerializer._fromJsonObject(item)),
            };
    },

    _toJsonObject(self: MsgPublishOut): any {
        return {
            'msgs': self.msgs.map((item) => MsgPublishOutMsgSerializer._toJsonObject(item)),
            };
    }
}