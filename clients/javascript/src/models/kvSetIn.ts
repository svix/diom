// this file is @generated
import {
    type OperationBehavior,
    OperationBehaviorSerializer,
} from './operationBehavior';

export interface KvSetIn {
    value: number[];
    /** Time to live in milliseconds */
    ttl?: number | null;
    behavior?: OperationBehavior;
}

export interface KvSetIn_ {
    key: string;
    value: number[];
    /** Time to live in milliseconds */
    ttl?: number | null;
    behavior?: OperationBehavior;
}

export const KvSetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvSetIn_ {
        return {
            key: object['key'],
            value: object['value'],
            ttl: object['ttl'],
            behavior: object['behavior'] != null ? OperationBehaviorSerializer._fromJsonObject(object['behavior']): undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvSetIn_): any {
        return {
            'key': self.key,
            'value': self.value,
            'ttl': self.ttl,
            'behavior': self.behavior != null ? OperationBehaviorSerializer._toJsonObject(self.behavior) : undefined,
        };
    }
}