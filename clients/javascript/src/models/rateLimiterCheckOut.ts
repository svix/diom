// this file is @generated
import {
    type RateLimitStatus,
    RateLimitStatusSerializer,
} from './rateLimitStatus';

export interface RateLimiterCheckOut {
    /** Whether the request is allowed */
    status: RateLimitStatus;
    /** Number of tokens remaining */
    remaining: number;
    /** Seconds until enough tokens are available (only present when allowed is false) */
    retryAfter?: number | null;
}

export const RateLimiterCheckOutSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimiterCheckOut {
        return {
            status: RateLimitStatusSerializer._fromJsonObject(object['status']),
            remaining: object['remaining'],
            retryAfter: object['retry_after'],
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimiterCheckOut): any {
        return {
            'status': RateLimitStatusSerializer._toJsonObject(self.status),
            'remaining': self.remaining,
            'retry_after': self.retryAfter,
        };
    }
}