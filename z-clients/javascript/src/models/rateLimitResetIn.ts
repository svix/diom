// this file is @generated
import {
    type RateLimitConfig,
    RateLimitConfigSerializer,
} from './rateLimitConfig';

export interface RateLimitResetIn {
    namespace?: string | null;
    key: string;
    /** Rate limiter configuration */
    config: RateLimitConfig;
}

export const RateLimitResetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitResetIn {
        return {
            namespace: object['namespace'],
            key: object['key'],
            config: RateLimitConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitResetIn): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'config': RateLimitConfigSerializer._toJsonObject(self.config),
        };
    }
}