// this file is @generated
import {
    type OperationBehavior,
    OperationBehaviorSerializer,
} from './operationBehavior';

export interface KvSetIn {
    namespace?: string | null;
    value: number[];
    /** Time to live in milliseconds */
    ttl?: number | null;
    behavior?: OperationBehavior;
    /**
     * If set, the write only succeeds when the stored version matches this value.
     * Use the `version` field from a prior `get` response.
     */
    version?: number | null;
}

export interface KvSetIn_ {
    namespace?: string | null;
    key: string;
    value: number[];
    /** Time to live in milliseconds */
    ttl?: number | null;
    behavior?: OperationBehavior;
    /**
     * If set, the write only succeeds when the stored version matches this value.
     * Use the `version` field from a prior `get` response.
     */
    version?: number | null;
}

export const KvSetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): KvSetIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            value: object['value'],
            ttl: object['ttl'],
            behavior: object['behavior'] != null ? OperationBehaviorSerializer._fromJsonObject(object['behavior']): undefined,
            version: object['version'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvSetIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'value': self.value,
            'ttl': self.ttl,
            'behavior': self.behavior != null ? OperationBehaviorSerializer._toJsonObject(self.behavior) : undefined,
            'version': self.version,
        };
    }
}