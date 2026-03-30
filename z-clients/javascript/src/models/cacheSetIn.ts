// this file is @generated

export interface CacheSetIn {
    namespace?: string | null;
    value: number[];
    /** Time to live in milliseconds */
    ttlMs: number;
}

export interface CacheSetIn_ {
    namespace?: string | null;
    key: string;
    value: number[];
    /** Time to live in milliseconds */
    ttlMs: number;
}

export const CacheSetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheSetIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            value: object['value'],
            ttlMs: object['ttl_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheSetIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'value': self.value,
            'ttl_ms': self.ttlMs,
        };
    }
}