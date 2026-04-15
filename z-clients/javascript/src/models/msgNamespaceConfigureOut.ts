// this file is @generated
import {
    type Retention,
    RetentionSerializer,
} from './retention';

export interface MsgNamespaceConfigureOut {
    name: string;
    retention: Retention;
    created: Date;
    updated: Date;
}

export const MsgNamespaceConfigureOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceConfigureOut {
        return {
            name: object['name'],
            retention: RetentionSerializer._fromJsonObject(object['retention']),
            created: new Date(Number(object['created'])),
            updated: new Date(Number(object['updated'])),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceConfigureOut): any {
        return {
            'name': self.name,
            'retention': RetentionSerializer._toJsonObject(self.retention),
            'created': self.created.getTime(),
            'updated': self.updated.getTime(),
        };
    }
}