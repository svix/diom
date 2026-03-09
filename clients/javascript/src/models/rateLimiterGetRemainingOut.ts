// this file is @generated

export interface RateLimiterGetRemainingOut {
    /** Number of tokens remaining */
    remaining: number;
    /** Seconds until at least one token is available (only present when remaining is 0) */
    retryAfter?: number | null;
}

export const RateLimiterGetRemainingOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimiterGetRemainingOut {
        return {
            remaining: object['remaining'],
            retryAfter: object['retry_after'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimiterGetRemainingOut): any {
        return {
            'remaining': self.remaining,
            'retry_after': self.retryAfter,
        };
    }
}