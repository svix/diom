// this file is @generated

export interface RateLimitTokenBucketConfig {
    /** Maximum capacity of the bucket */
    capacity: number;
    /** Number of tokens to add per refill interval */
    refillAmount: number;
    /** Interval in milliseconds between refills (minimum 1 millisecond) */
    refillIntervalMs?: number;
}

export const RateLimitTokenBucketConfigSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitTokenBucketConfig {
        return {
            capacity: object['capacity'],
            refillAmount: object['refill_amount'],
            refillIntervalMs: object['refill_interval_ms'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitTokenBucketConfig): any {
        return {
            'capacity': self.capacity,
            'refill_amount': self.refillAmount,
            'refill_interval_ms': self.refillIntervalMs,
        };
    }
}