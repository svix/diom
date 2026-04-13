// this file is @generated
import {
    type Retention,
    RetentionSerializer,
} from './retention';

export interface MsgNamespaceCreateOut {
    name: string;
    retention: Retention;
    created: Date;
    updated: Date;
}

export const MsgNamespaceCreateOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceCreateOut {
        return {
            name: object['name'],
            retention: RetentionSerializer._fromJsonObject(object['retention']),
            created: new Date(object['created']),
            updated: new Date(object['updated']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceCreateOut): any {
        return {
            'name': self.name,
            'retention': RetentionSerializer._toJsonObject(self.retention),
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}