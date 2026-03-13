// this file is @generated
import {
    type RateLimitTokenBucketConfig,
    RateLimitTokenBucketConfigSerializer,
} from './rateLimitTokenBucketConfig';

export interface RateLimitGetRemainingIn {
    key: string;
    /** Rate limiter configuration */
    config: RateLimitTokenBucketConfig;
}

export const RateLimitGetRemainingInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitGetRemainingIn {
        return {
            key: object['key'],
            config: RateLimitTokenBucketConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitGetRemainingIn): any {
        return {
            'key': self.key,
            'config': RateLimitTokenBucketConfigSerializer._toJsonObject(self.config),
        };
    }
}