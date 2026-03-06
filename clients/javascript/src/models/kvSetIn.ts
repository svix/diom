// this file is @generated
import {
    type OperationBehavior,
    OperationBehaviorSerializer,
} from './operationBehavior';





export interface KvSetIn {
    key: string;
    value: number[];
    /** Time to live in milliseconds */
    ttl?: number | null;
    behavior?: OperationBehavior;
}

export const KvSetInSerializer = {
    _fromJsonObject(object: any): KvSetIn {
        return {
            key: object['key'],
            value: object['value'],
            ttl: object['ttl'],
            behavior: object['behavior'] != null ? OperationBehaviorSerializer._fromJsonObject(object['behavior']): undefined,
            };
    },

    _toJsonObject(self: KvSetIn): any {
        return {
            'key': self.key,
            'value': self.value,
            'ttl': self.ttl,
            'behavior': self.behavior != null ? OperationBehaviorSerializer._toJsonObject(self.behavior) : undefined,
            };
    }
}