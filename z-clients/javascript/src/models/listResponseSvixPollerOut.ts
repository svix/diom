// this file is @generated
import {
    type SvixPollerOut,
    SvixPollerOutSerializer,
} from './svixPollerOut';

export interface ListResponseSvixPollerOut {
    data: SvixPollerOut[];
    iterator?: string | null;
    prevIterator?: string | null;
    done: boolean;
}

export const ListResponseSvixPollerOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ListResponseSvixPollerOut {
        return {
            data: object['data'].map((item: SvixPollerOut) => SvixPollerOutSerializer._fromJsonObject(item)),
            iterator: object['iterator'],
            prevIterator: object['prev_iterator'],
            done: object['done'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ListResponseSvixPollerOut): any {
        return {
            'data': self.data.map((item) => SvixPollerOutSerializer._toJsonObject(item)),
            'iterator': self.iterator,
            'prev_iterator': self.prevIterator,
            'done': self.done,
        };
    }
}