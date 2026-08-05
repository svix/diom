// this file is @generated
import {
    type FifoMsgOut,
    FifoMsgOutSerializer,
} from './fifoMsgOut';

export interface MsgFifoReceiveOut {
    msgs: FifoMsgOut[];
}

export const MsgFifoReceiveOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgFifoReceiveOut {
        return {
            msgs: object['msgs'].map((item: FifoMsgOut) => FifoMsgOutSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgFifoReceiveOut): any {
        return {
            'msgs': self.msgs.map((item) => FifoMsgOutSerializer._toJsonObject(item)),
        };
    }
}