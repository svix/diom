// this file is @generated
import {
    type AdminAuthTokenOut,
    AdminAuthTokenOutSerializer,
} from './adminAuthTokenOut';

export interface ListResponseAdminAuthTokenOut {
    data: AdminAuthTokenOut[];
    iterator?: string | null;
    prevIterator?: string | null;
    done: boolean;
}

export const ListResponseAdminAuthTokenOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ListResponseAdminAuthTokenOut {
        return {
            data: object['data'].map((item: AdminAuthTokenOut) => AdminAuthTokenOutSerializer._fromJsonObject(item)),
            iterator: object['iterator'],
            prevIterator: object['prev_iterator'],
            done: object['done'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ListResponseAdminAuthTokenOut): any {
        return {
            'data': self.data.map((item) => AdminAuthTokenOutSerializer._toJsonObject(item)),
            'iterator': self.iterator,
            'prev_iterator': self.prevIterator,
            'done': self.done,
        };
    }
}