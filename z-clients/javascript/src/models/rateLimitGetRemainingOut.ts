// this file is @generated

export interface RateLimitGetRemainingOut {
    /** Number of tokens remaining */
    remaining: number;
    /** Milliseconds until at least one token is available (only present when remaining is 0) */
    retryAfter?: Date | null;
}

export const RateLimitGetRemainingOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetRemainingOut {
        return {
            remaining: object['remaining'],
            retryAfter: object['retry_after_ms'] ? new Date(object['retry_after_ms']) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetRemainingOut): any {
        return {
            'remaining': self.remaining,
            'retry_after_ms': self.retryAfter,
        };
    }
}