// this file is @generated

export interface IdempotencyStartIn {
    namespace?: string | null;
    /** TTL in seconds for the lock/response */
    ttl: number;
}

export interface IdempotencyStartIn_ {
    namespace?: string | null;
    key: string;
    /** TTL in seconds for the lock/response */
    ttl: number;
}

export const IdempotencyStartInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyStartIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            ttl: object['ttl'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyStartIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'ttl': self.ttl,
        };
    }
}