// this file is @generated

export interface CacheSetIn {
    namespace?: string | null;
    /** Time to live in milliseconds */
    ttlMs: number;
}

export interface CacheSetIn_ {
    namespace?: string | null;
    key: string;
    value: Uint8Array;
    /** Time to live in milliseconds */
    ttlMs: number;
}

export const CacheSetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): CacheSetIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            value: new Uint8Array(object['value']),
            ttlMs: object['ttl_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: CacheSetIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'value': Array.from(self.value),
            'ttl_ms': self.ttlMs,
        };
    }
}