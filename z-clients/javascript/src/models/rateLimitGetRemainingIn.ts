// this file is @generated
import {
    type RateLimitConfig,
    RateLimitConfigSerializer,
} from './rateLimitConfig';

export interface RateLimitGetRemainingIn {
    namespace?: string | null;
    key: string;
    /** Rate limiter configuration */
    config: RateLimitConfig;
}

export const RateLimitGetRemainingInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetRemainingIn {
        return {
            namespace: object['namespace'],
            key: object['key'],
            config: RateLimitConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetRemainingIn): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'config': RateLimitConfigSerializer._toJsonObject(self.config),
        };
    }
}