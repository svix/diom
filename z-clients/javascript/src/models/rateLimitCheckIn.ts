// this file is @generated
import {
    type RateLimitConfig,
    RateLimitConfigSerializer,
} from './rateLimitConfig';

export interface RateLimitCheckIn {
    namespace?: string | null;
    key: string;
    /** Number of tokens to consume (default: 1) */
    tokens?: number;
    /** Rate limiter configuration */
    config: RateLimitConfig;
}

export const RateLimitCheckInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitCheckIn {
        return {
            namespace: object['namespace'],
            key: object['key'],
            tokens: object['tokens'],
            config: RateLimitConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitCheckIn): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'tokens': self.tokens,
            'config': RateLimitConfigSerializer._toJsonObject(self.config),
        };
    }
}