// this file is @generated
import {
    type AuthTokenOut,
    AuthTokenOutSerializer,
} from './authTokenOut';

export interface ListResponseAuthTokenOut {
    data: AuthTokenOut[];
    iterator?: string | null;
    prevIterator?: string | null;
    done: boolean;
}

export const ListResponseAuthTokenOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ListResponseAuthTokenOut {
        return {
            data: object['data'].map((item: AuthTokenOut) => AuthTokenOutSerializer._fromJsonObject(item)),
            iterator: object['iterator'],
            prevIterator: object['prev_iterator'],
            done: object['done'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ListResponseAuthTokenOut): any {
        return {
            'data': self.data.map((item) => AuthTokenOutSerializer._toJsonObject(item)),
            'iterator': self.iterator,
            'prev_iterator': self.prevIterator,
            'done': self.done,
        };
    }
}