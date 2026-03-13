// this file is @generated

export interface RateLimitCheckIn {
    key: string;
    /** Number of tokens to consume (default: 1) */
    tokens?: number;
    /** Maximum capacity of the bucket */
    capacity: number;
    /** Number of tokens to add per refill interval */
    refillAmount: number;
    /** Interval in milliseconds between refills (minimum 1 millisecond) */
    refillIntervalMillis?: number;
}

export const RateLimitCheckInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCheckIn {
        return {
            key: object['key'],
            tokens: object['tokens'],
            capacity: object['capacity'],
            refillAmount: object['refill_amount'],
            refillIntervalMillis: object['refill_interval_millis'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCheckIn): any {
        return {
            'key': self.key,
            'tokens': self.tokens,
            'capacity': self.capacity,
            'refill_amount': self.refillAmount,
            'refill_interval_millis': self.refillIntervalMillis,
        };
    }
}