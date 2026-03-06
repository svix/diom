// this file is @generated
import {
    type StreamMsgOut,
    StreamMsgOutSerializer,
} from './streamMsgOut';

export interface MsgStreamReceiveOut {
    msgs: StreamMsgOut[];
}

export const MsgStreamReceiveOutSerializer = {
    _fromJsonObject(object: any): MsgStreamReceiveOut {
        return {
            msgs: object['msgs'].map((item: StreamMsgOut) => StreamMsgOutSerializer._fromJsonObject(item)),
            };
    },

    _toJsonObject(self: MsgStreamReceiveOut): any {
        return {
            'msgs': self.msgs.map((item) => StreamMsgOutSerializer._toJsonObject(item)),
            };
    }
}