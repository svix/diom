// this file is @generated

export interface RateLimitGetRemainingOut {
    /** Number of tokens remaining */
    remaining: number;
    /** Seconds until at least one token is available (only present when remaining is 0) */
    retryAfter?: number | null;
}

export const RateLimitGetRemainingOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetRemainingOut {
        return {
            remaining: object['remaining'],
            retryAfter: object['retry_after'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetRemainingOut): any {
        return {
            'remaining': self.remaining,
            'retry_after': self.retryAfter,
        };
    }
}