// this file is @generated

export interface IdempotencyCompleteIn {
    namespace?: string | null;
    /** The response to cache */
    response: number[];
    /** How long to keep the idempotency response for. */
    ttlMs: number;
}

export interface IdempotencyCompleteIn_ {
    namespace?: string | null;
    key: string;
    /** The response to cache */
    response: number[];
    /** How long to keep the idempotency response for. */
    ttlMs: number;
}

export const IdempotencyCompleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCompleteIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            response: object['response'],
            ttlMs: object['ttl_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCompleteIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'response': self.response,
            'ttl_ms': self.ttlMs,
        };
    }
}