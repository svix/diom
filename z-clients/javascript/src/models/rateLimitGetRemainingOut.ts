// this file is @generated
import { Temporal } from 'temporal-polyfill-lite';

export interface RateLimitGetRemainingOut {
    /** Number of tokens remaining */
    remaining: number;
    /** Milliseconds until at least one token is available (only present when remaining is 0) */
    retryAfter?: Temporal.Duration | null;
}

export const RateLimitGetRemainingOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetRemainingOut {
        return {
            remaining: object['remaining'],
            retryAfter: object['retry_after_ms'] != null ? Temporal.Duration.from({ milliseconds: object['retry_after_ms'] }) : undefined,
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetRemainingOut): any {
        return {
            'remaining': self.remaining,
            'retry_after_ms': self.retryAfter != null ? self.retryAfter.total('millisecond') : undefined,
        };
    }
}