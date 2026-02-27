// this file is @generated
import {
    type StreamMsgOut,
    StreamMsgOutSerializer,
} from './streamMsgOut';





export interface StreamReceiveOut {
    msgs: StreamMsgOut[];
}

export const StreamReceiveOutSerializer = {
    _fromJsonObject(object: any): StreamReceiveOut {
        return {
            msgs: object['msgs'].map((item: StreamMsgOut) => StreamMsgOutSerializer._fromJsonObject(item)),
            };
    },

    _toJsonObject(self: StreamReceiveOut): any {
        return {
            'msgs': self.msgs.map((item) => StreamMsgOutSerializer._toJsonObject(item)),
            };
    }
}