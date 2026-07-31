// this file is @generated
import {
    type SinkOut,
    SinkOutSerializer,
} from './sinkOut';

export interface ListResponseSinkOut {
    data: SinkOut[];
    iterator?: string | null;
    prevIterator?: string | null;
    done: boolean;
}

export const ListResponseSinkOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ListResponseSinkOut {
        return {
            data: object['data'].map((item: SinkOut) => SinkOutSerializer._fromJsonObject(item)),
            iterator: object['iterator'],
            prevIterator: object['prev_iterator'],
            done: object['done'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ListResponseSinkOut): any {
        return {
            'data': self.data.map((item) => SinkOutSerializer._toJsonObject(item)),
            'iterator': self.iterator,
            'prev_iterator': self.prevIterator,
            'done': self.done,
        };
    }
}