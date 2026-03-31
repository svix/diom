// this file is @generated
import {
    type AdminAccessPolicyOut,
    AdminAccessPolicyOutSerializer,
} from './adminAccessPolicyOut';

export interface ListResponseAdminAccessPolicyOut {
    data: AdminAccessPolicyOut[];
    iterator?: string | null;
    prevIterator?: string | null;
    done: boolean;
}

export const ListResponseAdminAccessPolicyOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): ListResponseAdminAccessPolicyOut {
        return {
            data: object['data'].map((item: AdminAccessPolicyOut) => AdminAccessPolicyOutSerializer._fromJsonObject(item)),
            iterator: object['iterator'],
            prevIterator: object['prev_iterator'],
            done: object['done'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: ListResponseAdminAccessPolicyOut): any {
        return {
            'data': self.data.map((item) => AdminAccessPolicyOutSerializer._toJsonObject(item)),
            'iterator': self.iterator,
            'prev_iterator': self.prevIterator,
            'done': self.done,
        };
    }
}