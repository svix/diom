// this file is @generated
import {
    type RateLimitStatus,
    RateLimitStatusSerializer,
} from './rateLimitStatus';





export interface RateLimiterCheckOut {
    /** Number of tokens remaining */
    remaining: number;
/** Seconds until enough tokens are available (only present when allowed is false) */
    retryAfter?: number | null;
/** Whether the request is allowed */
    status: RateLimitStatus;
}

export const RateLimiterCheckOutSerializer = {
    _fromJsonObject(object: any): RateLimiterCheckOut {
        return {
            remaining: object['remaining'],
            retryAfter: object['retry_after'],
            status: RateLimitStatusSerializer._fromJsonObject(object['status']),
            };
    },

    _toJsonObject(self: RateLimiterCheckOut): any {
        return {
            'remaining': self.remaining,
            'retry_after': self.retryAfter,
            'status': RateLimitStatusSerializer._toJsonObject(self.status),
            };
    }
}