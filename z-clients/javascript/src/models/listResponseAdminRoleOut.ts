// this file is @generated
import {
    type AdminRoleOut,
    AdminRoleOutSerializer,
} from './adminRoleOut';

export interface ListResponseAdminRoleOut {
    data: AdminRoleOut[];
    iterator?: string | null;
    prevIterator?: string | null;
    done: boolean;
}

export const ListResponseAdminRoleOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ListResponseAdminRoleOut {
        return {
            data: object['data'].map((item: AdminRoleOut) => AdminRoleOutSerializer._fromJsonObject(item)),
            iterator: object['iterator'],
            prevIterator: object['prev_iterator'],
            done: object['done'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ListResponseAdminRoleOut): any {
        return {
            'data': self.data.map((item) => AdminRoleOutSerializer._toJsonObject(item)),
            'iterator': self.iterator,
            'prev_iterator': self.prevIterator,
            'done': self.done,
        };
    }
}