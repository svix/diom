// this file is @generated

export interface RateLimiterTokenBucketConfig {
    /** Maximum capacity of the bucket */
    capacity: number;
    /** Number of tokens to add per refill interval */
    refillAmount: number;
    /** Interval in seconds between refills (minimum 1 second) */
    refillInterval?: number;
}

export const RateLimiterTokenBucketConfigSerializer = {
    _fromJsonObject(object: any): RateLimiterTokenBucketConfig {
        return {
            capacity: object['capacity'],
            refillAmount: object['refill_amount'],
            refillInterval: object['refill_interval'],
        };
    },

    _toJsonObject(self: RateLimiterTokenBucketConfig): any {
        return {
            'capacity': self.capacity,
            'refill_amount': self.refillAmount,
            'refill_interval': self.refillInterval,
        };
    }
}