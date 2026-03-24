// this file is @generated
import {
    type QueueMsgOut,
    QueueMsgOutSerializer,
} from './queueMsgOut';

export interface MsgQueueReceiveOut {
    msgs: QueueMsgOut[];
}

export const MsgQueueReceiveOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgQueueReceiveOut {
        return {
            msgs: object['msgs'].map((item: QueueMsgOut) => QueueMsgOutSerializer._fromJsonObject(item)),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgQueueReceiveOut): any {
        return {
            'msgs': self.msgs.map((item) => QueueMsgOutSerializer._toJsonObject(item)),
        };
    }
}