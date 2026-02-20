// this file is @generated
import {
    type OperationBehavior,
    OperationBehaviorSerializer,
} from './operationBehavior';





export interface KvSetIn {
    behavior?: OperationBehavior;
key: string;
/** Time to live in milliseconds */
    ttl?: number | null;
value: number[];
}

export const KvSetInSerializer = {
    _fromJsonObject(object: any): KvSetIn {
        return {
            behavior: object['behavior'] != null ? OperationBehaviorSerializer._fromJsonObject(object['behavior']): undefined,
            key: object['key'],
            ttl: object['ttl'],
            value: object['value'],
            };
    },

    _toJsonObject(self: KvSetIn): any {
        return {
            'behavior': self.behavior != null ? OperationBehaviorSerializer._toJsonObject(self.behavior) : undefined,
            'key': self.key,
            'ttl': self.ttl,
            'value': self.value,
            };
    }
}