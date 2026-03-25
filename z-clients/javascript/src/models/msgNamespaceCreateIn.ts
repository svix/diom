// this file is @generated
import {
    type Retention,
    RetentionSerializer,
} from './retention';

export interface MsgNamespaceCreateIn {
    retention?: Retention;
}

export interface MsgNamespaceCreateIn_ {
    name: string;
    retention?: Retention;
}

export const MsgNamespaceCreateInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceCreateIn_ {
        return {
            name: object['name'],
            retention: object['retention'] != null ? RetentionSerializer._fromJsonObject(object['retention']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceCreateIn_): any {
        return {
            'name': self.name,
            'retention': self.retention != null ? RetentionSerializer._toJsonObject(self.retention) : undefined,
        };
    }
}