// this file is @generated
import {
    type Retention,
    RetentionSerializer,
} from './retention';

export interface MsgNamespaceGetOut {
    name: string;
    retention: Retention;
    created: number;
    updated: number;
}

export const MsgNamespaceGetOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceGetOut {
        return {
            name: object['name'],
            retention: RetentionSerializer._fromJsonObject(object['retention']),
            created: object['created'],
            updated: object['updated'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceGetOut): any {
        return {
            'name': self.name,
            'retention': RetentionSerializer._toJsonObject(self.retention),
            'created': self.created,
            'updated': self.updated,
        };
    }
}