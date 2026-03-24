// this file is @generated
import {
    type StreamMsgOut,
    StreamMsgOutSerializer,
} from './streamMsgOut';

export interface MsgStreamReceiveOut {
    msgs: StreamMsgOut[];
}

export const MsgStreamReceiveOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgStreamReceiveOut {
        return {
            msgs: object['msgs'].map((item: StreamMsgOut) => StreamMsgOutSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgStreamReceiveOut): any {
        return {
            'msgs': self.msgs.map((item) => StreamMsgOutSerializer._toJsonObject(item)),
        };
    }
}