// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface RateLimitCheckOut {
    /** Whether the request is allowed */
    allowed: boolean;
    /** Number of tokens remaining */
    remaining: number;
    /** Milliseconds until enough tokens are available (only present when allowed is false) */
    retryAfter?: Temporal.Duration | null;
}

export const RateLimitCheckOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCheckOut {
        return {
            allowed: object['allowed'],
            remaining: object['remaining'],
            retryAfter: object['retry_after_ms'] != null ? Temporal.Duration.from({ milliseconds: object['retry_after_ms'] }) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCheckOut): any {
        return {
            'allowed': self.allowed,
            'remaining': self.remaining,
            'retry_after_ms': self.retryAfter != null ? self.retryAfter.total('millisecond') : undefined,
        };
    }
}