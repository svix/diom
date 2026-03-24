// this file is @generated
import {
    type RateLimitTokenBucketConfig,
    RateLimitTokenBucketConfigSerializer,
} from './rateLimitTokenBucketConfig';

export interface RateLimitResetIn {
    namespace?: string | null;
    key: string;
    /** Rate limiter configuration */
    config: RateLimitTokenBucketConfig;
}

export const RateLimitResetInSerializer = {
    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _fromJsonObject(object: any): RateLimitResetIn {
        return {
            namespace: object['namespace'],
            key: object['key'],
            config: RateLimitTokenBucketConfigSerializer._fromJsonObject(object['config']),
        };
    },

    // biome-ignore lint/suspicious/noExplicitAny: intentional any
    _toJsonObject(self: RateLimitResetIn): any {
        return {
            'namespace': self.namespace,
            'key': self.key,
            'config': RateLimitTokenBucketConfigSerializer._toJsonObject(self.config),
        };
    }
}