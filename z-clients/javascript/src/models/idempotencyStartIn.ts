// this file is @generated

export interface IdempotencyStartIn {
    namespace?: string | null;
    /** TTL in milliseconds for the lock/response */
    ttlMs: number;
}

export interface IdempotencyStartIn_ {
    namespace?: string | null;
    key: string;
    /** TTL in milliseconds for the lock/response */
    ttlMs: number;
}

export const IdempotencyStartInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyStartIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            ttlMs: object['ttl_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyStartIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'ttl_ms': self.ttlMs,
        };
    }
}