// this file is @generated
import {
    type OperationBehavior,
    OperationBehaviorSerializer,
} from './operationBehavior';





export interface KvSetIn {
    key: string;
/** Time to live in milliseconds */
    ttl?: number | null;
behavior?: OperationBehavior;
value: number[];
}

export const KvSetInSerializer = {
    _fromJsonObject(object: any): KvSetIn {
        return {
            key: object['key'],
            ttl: object['ttl'],
            behavior: object['behavior'] != null ? OperationBehaviorSerializer._fromJsonObject(object['behavior']): undefined,
            value: object['value'],
            };
    },

    _toJsonObject(self: KvSetIn): any {
        return {
            'key': self.key,
            'ttl': self.ttl,
            'behavior': self.behavior != null ? OperationBehaviorSerializer._toJsonObject(self.behavior) : undefined,
            'value': self.value,
            };
    }
}