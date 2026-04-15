// this file is @generated
import {
    type Retention,
    RetentionSerializer,
} from './retention';

export interface MsgNamespaceConfigureIn {
    retention?: Retention;
}

export interface MsgNamespaceConfigureIn_ {
    name: string;
    retention?: Retention;
}

export const MsgNamespaceConfigureInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): MsgNamespaceConfigureIn_ {
        return {
            name: object['name'],
            retention: object['retention'] != null ? RetentionSerializer._fromJsonObject(object['retention']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: MsgNamespaceConfigureIn_): any {
        return {
            'name': self.name,
            'retention': self.retention != null ? RetentionSerializer._toJsonObject(self.retention) : undefined,
        };
    }
}