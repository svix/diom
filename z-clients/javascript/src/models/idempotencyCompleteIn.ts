// this file is @generated

export interface IdempotencyCompleteIn {
    namespace?: string | null;
    /** The response to cache */
    response: Uint8Array;
    /** Optional metadata to store alongside the response */
    context?: { [key: string]: string } | null;
    /** How long to keep the idempotency response for. */
    ttlMs: number;
}

export interface IdempotencyCompleteIn_ {
    namespace?: string | null;
    key: string;
    /** The response to cache */
    response: Uint8Array;
    /** Optional metadata to store alongside the response */
    context?: { [key: string]: string } | null;
    /** How long to keep the idempotency response for. */
    ttlMs: number;
}

export const IdempotencyCompleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCompleteIn_ {
        return {
            namespace: object['namespace'],
            key: object['key'],
            response: new Uint8Array(object['response']),
            context: object['context'],
            ttlMs: object['ttl_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCompleteIn_): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'response': Array.from(self.response),
            'context': self.context,
            'ttl_ms': self.ttlMs,
        };
    }
}