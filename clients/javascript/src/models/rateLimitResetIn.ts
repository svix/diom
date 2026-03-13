// this file is @generated
import {
    type RateLimitTokenBucketConfig,
    RateLimitTokenBucketConfigSerializer,
} from './rateLimitTokenBucketConfig';

export interface RateLimitResetIn {
    key: string;
    /** Rate limiter configuration */
    config: RateLimitTokenBucketConfig;
}

export const RateLimitResetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitResetIn {
        return {
            key: object['key'],
            config: RateLimitTokenBucketConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitResetIn): any {
        return {
            'key': self.key,
            'config': RateLimitTokenBucketConfigSerializer._toJsonObject(self.config),
        };
    }
}