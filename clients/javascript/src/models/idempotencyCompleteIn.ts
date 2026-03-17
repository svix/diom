// this file is @generated

export interface IdempotencyCompleteIn {
    /** The response to cache */
    response: number[];
    /** TTL in seconds for the cached response */
    ttl: number;
}

export interface IdempotencyCompleteIn_ {
    key: string;
    /** The response to cache */
    response: number[];
    /** TTL in seconds for the cached response */
    ttl: number;
}

export const IdempotencyCompleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCompleteIn_ {
        return {
            key: object['key'],
            response: object['response'],
            ttl: object['ttl'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCompleteIn_): any {
        return {
            'key': self.key,
            'response': self.response,
            'ttl': self.ttl,
        };
    }
}