// this file is @generated

export interface RateLimitGetRemainingIn {
    key: string;
    /** Maximum capacity of the bucket */
    capacity: number;
    /** Number of tokens to add per refill interval */
    refillAmount: number;
    /** Interval in milliseconds between refills (minimum 1 millisecond) */
    refillIntervalMillis?: number;
}

export const RateLimitGetRemainingInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetRemainingIn {
        return {
            key: object['key'],
            capacity: object['capacity'],
            refillAmount: object['refill_amount'],
            refillIntervalMillis: object['refill_interval_millis'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetRemainingIn): any {
        return {
            'key': self.key,
            'capacity': self.capacity,
            'refill_amount': self.refillAmount,
            'refill_interval_millis': self.refillIntervalMillis,
        };
    }
}