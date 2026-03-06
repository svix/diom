// this file is @generated
import {
    type QueueMsgOut,
    QueueMsgOutSerializer,
} from './queueMsgOut';

export interface MsgQueueReceiveOut {
    msgs: QueueMsgOut[];
}

export const MsgQueueReceiveOutSerializer = {
    _fromJsonObject(object: any): MsgQueueReceiveOut {
        return {
            msgs: object['msgs'].map((item: QueueMsgOut) => QueueMsgOutSerializer._fromJsonObject(item)),
        };
    },

    _toJsonObject(self: MsgQueueReceiveOut): any {
        return {
            'msgs': self.msgs.map((item) => QueueMsgOutSerializer._toJsonObject(item)),
        };
    }
}