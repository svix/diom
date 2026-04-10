// this file is @generated

export interface RateLimitCheckOut {
    /** Whether the request is allowed */
    allowed: boolean;
    /** Number of tokens remaining */
    remaining: number;
    /** Milliseconds until enough tokens are available (only present when allowed is false) */
    retryAfter?: Date | null;
}

export const RateLimitCheckOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCheckOut {
        return {
            allowed: object['allowed'],
            remaining: object['remaining'],
            retryAfterMs: object['retry_after_ms'] ? new Date(object['retry_after_ms']) : null,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCheckOut): any {
        return {
            'allowed': self.allowed,
            'remaining': self.remaining,
            'retry_after_ms': self.retryAfterMs,
        };
    }
}