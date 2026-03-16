// this file is @generated

export interface IdempotencyCompleteIn {
    key: string;
    /** The response to cache */
    response: number[];
    /** TTL in seconds for the cached response */
    ttl: number;
}

export const IdempotencyCompleteInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): IdempotencyCompleteIn {
        return {
            key: object['key'],
            response: object['response'],
            ttl: object['ttl'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: IdempotencyCompleteIn): any {
        return {
            'key': self.key,
            'response': self.response,
            'ttl': self.ttl,
        };
    }
}