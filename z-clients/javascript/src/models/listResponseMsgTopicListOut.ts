// this file is @generated
import {
    type MsgTopicListOut,
    MsgTopicListOutSerializer,
} from './msgTopicListOut';

export interface ListResponseMsgTopicListOut {
    data: MsgTopicListOut[];
    iterator?: string | null;
    prevIterator?: string | null;
    done: boolean;
}

export const ListResponseMsgTopicListOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ListResponseMsgTopicListOut {
        return {
            data: object['data'].map((item: MsgTopicListOut) => MsgTopicListOutSerializer._fromJsonObject(item)),
            iterator: object['iterator'],
            prevIterator: object['prev_iterator'],
            done: object['done'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ListResponseMsgTopicListOut): any {
        return {
            'data': self.data.map((item) => MsgTopicListOutSerializer._toJsonObject(item)),
            'iterator': self.iterator,
            'prev_iterator': self.prevIterator,
            'done': self.done,
        };
    }
}