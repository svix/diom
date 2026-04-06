// this file is @generated
import {
    type OperationBehavior,
    OperationBehaviorSerializer,
} from './operationBehavior';

export interface KvSetIn {
    namespace?: string | null;
    /** Time to live in milliseconds */
    ttlMs?: number | null;
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
    value: Uint8Array;
    /** Time to live in milliseconds */
    ttlMs?: number | null;
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
            value: new Uint8Array(object['value']),
            ttlMs: object['ttl_ms'],
            behavior: object['behavior'] != null ? OperationBehaviorSerializer._fromJsonObject(object['behavior']): undefined,
            version: object['version'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: KvSetIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'value': Array.from(self.value),
            'ttl_ms': self.ttlMs,
            'behavior': self.behavior != null ? OperationBehaviorSerializer._toJsonObject(self.behavior) : undefined,
            'version': self.version,
        };
    }
}